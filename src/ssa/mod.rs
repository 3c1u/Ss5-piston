use graphics::math::Matrix2d;
use graphics::Graphics;

use gfx_core::command::Buffer;
use gfx_core::factory::Factory;
use gfx_core::Resources;
use gfx_texture::{Flip, Texture, TextureContext, TextureSettings};

use std::path::{Path, PathBuf};

use std::fs::File;
use std::io::{BufReader, Read};

use serde::Deserialize;
use serde_json::from_str;

mod blend_type;
use blend_type::BlendType;

mod frame_data;
use frame_data::FrameData;

#[derive(Copy, Clone, PartialEq)]
enum PlayStatus {
    Loop,
    Nonloop,
    Stop,
}

/// SSAJSONをロードし，再生します．
pub struct SsaJson<G>
where
    G: Graphics + 'static,
{
    // ルートパス．画像の探索パスです．
    root_path: PathBuf,
    images_buf: Vec<G::Texture>,
    motions: Vec<Motion>,

    frame: usize,
    frame_count: usize,
    time_elapsed: f64,
    cycle: f64,

    play_status: PlayStatus,
}

#[derive(Clone, Debug, Deserialize)]
struct Motion {
    /// 画像データのパス．
    pub images: Vec<String>,
    /// モーション名．
    pub name: String,
    /// アニメーション
    pub animation: Animation,
}

#[derive(Clone, Debug, Deserialize)]
struct Animation {
    /// アニメーションの再生FPS．
    pub fps: f64,
    /// 基準枠の幅．
    #[serde(rename = "CanvasWidth")]
    pub canvas_width: f64,
    /// 基準枠の高さ．
    #[serde(rename = "CanvasHeight")]
    pub canvas_height: f64,
    /// 基準枠の原点X．
    #[serde(rename = "MarginWidth")]
    pub margin_width: f64,
    /// 基準枠の原点Y．
    #[serde(rename = "MarginHeight")]
    pub margin_height: f64,
    /// アニメーションに含まれるパーツ名
    pub parts: Vec<String>,
    pub ssa: Vec<Vec<FrameData>>,
}

impl<G> SsaJson<G>
where
    G: Graphics,
{
    /// SSAJSONファイルを読み込みます．
    pub fn open<P>(path: P) -> Option<SsaJson<G>>
    where
        P: AsRef<Path>,
        G: Graphics,
    {
        let path = path.as_ref();

        if !path.is_file() {
            return None;
        }

        let mut root_path = path.to_owned();
        root_path.pop();

        let mut ssa = SsaJson {
            root_path,
            images_buf: Vec::new(),
            motions: Vec::new(),
            frame: 0,
            frame_count: 1,
            time_elapsed: 0.0,
            cycle: 0.0,
            play_status: PlayStatus::Stop,
        };

        ssa.load_json(path)?;

        Some(ssa)
    }

    fn load_json(&mut self, path: &Path) -> Option<()> {
        // JSONファイルを読み込む
        let f = File::open(path).ok()?;
        let mut f = BufReader::new(f);
        let mut buf = String::new();
        f.read_to_string(&mut buf).ok()?;

        // モーションデータを取得
        let anim: Vec<Motion> = from_str(&buf).ok()?;
        self.motions = anim;

        Some(())
    }

    /// モーションを一度だけ再生します．
    pub fn play_once(&mut self) {
        self.frame = 0;
        self.play_status = PlayStatus::Nonloop;
    }

    /// モーションをループで再生します．
    pub fn play_looped(&mut self) {
        self.frame = 0;
        self.play_status = PlayStatus::Loop;
    }

    /// モーションを一時停止します．
    pub fn pause(&mut self) {
        self.play_status = PlayStatus::Stop;
    }

    /// モーションを停止します．
    pub fn stop(&mut self) {
        self.frame = 0;
        self.play_status = PlayStatus::Stop;
    }

    pub fn get_size_of(&self, motion: usize) -> (f64, f64) {
        (
            self.motions[motion].animation.canvas_width,
            self.motions[motion].animation.canvas_height,
        )
    }

    /// モーションを読み込み，テクスチャを確保します．
    pub fn allocate_texture_for<F, R, C>(&mut self, c: &mut TextureContext<F, R, C>, motion: usize)
    where
        R: Resources,
        F: Factory<R>,
        C: Buffer<R>,
        G::Texture: From<Texture<R>>,
    {
        // 画像を空にする
        self.images_buf.clear();

        let m = &self.motions[motion];
        self.cycle = 1.0 / m.animation.fps;
        self.frame_count = m.animation.ssa.len();

        for img in &m.images {
            let mut p = self.root_path.clone();
            p.push(img);

            if !p.is_file() {
                continue;
            }

            let t = Texture::from_path(c, &p, Flip::None, &TextureSettings::new())
                .expect("Failed to load texture.");

            self.images_buf.push(G::Texture::from(t));
        }
    }

    /// モーションを描画します．
    pub fn draw<'t, G_>(&mut self, transform: Matrix2d, g: &'t mut G_, motion: usize)
    where
        G_: Graphics,
    {
        use graphics::{DrawState, Image, Transformed};

        let motion = &self.motions[motion];
        let frame = &motion.animation.ssa[self.frame];

        let state = DrawState::new_alpha();

        let t = transform;

        for part in frame {
            let tex: &G_::Texture = unsafe {
                // I know this is the fucking same...
                &*(&self.images_buf[part.image_number as usize] as *const G::Texture
                    as *const G_::Texture)
            };

            let t = t
                .trans(part.position_x, part.position_y)
                .rot_rad(-part.angle)
                .scale(part.scale_x, part.scale_y)
                .trans(
                    -part.pivot_offset_x + 0.5 * part.source_width,
                    -part.pivot_offset_y + 0.5 * part.source_height,
                )
                .scale(
                    if part.flip_h == 1 { -1.0 } else { 1.0 },
                    if part.flip_v == 1 { -1.0 } else { 1.0 },
                )
                .trans(-0.5 * part.source_width, -0.5 * part.source_height);

            use graphics::draw_state::Blend;

            let state = state.blend(match part.blend_type {
                BlendType::Mix => Blend::Alpha,
                BlendType::Multiple => Blend::Multiply,
                BlendType::Additive => Blend::Screen,
                BlendType::Subtractive => Blend::Invert,
            });

            Image::new_color([1.0, 1.0, 1.0, part.opacity as f32])
                .src_rect([
                    part.source_left,
                    part.source_top,
                    part.source_width,
                    part.source_height,
                ])
                .draw(tex, &state, t, g);
        }
    }

    /// フレームの更新の際に呼び出すメソッド．
    pub fn update(&mut self, delta_time: f64) {
        if self.play_status == PlayStatus::Stop {
            return;
        }

        self.time_elapsed += delta_time;

        let frames = (self.time_elapsed / self.cycle).floor();
        self.time_elapsed -= frames * self.cycle;

        self.frame += frames as usize;

        if self.frame_count <= self.frame {
            match self.play_status {
                PlayStatus::Loop => {
                    self.frame %= self.frame_count;
                }
                PlayStatus::Nonloop => {
                    self.frame = 0;
                    self.play_status = PlayStatus::Stop;
                }
                _ => {}
            }
        }
    }
}
