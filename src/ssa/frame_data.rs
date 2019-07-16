use crate::ssa::BlendType;

#[derive(Clone, Debug, Deserialize)]
pub struct FrameData {
    pub part_number: i32,
    pub image_number: i32,
    pub source_left: f64,
    pub source_top: f64,
    pub source_width: f64,
    pub source_height: f64,

    pub position_x: f64,
    pub position_y: f64,
    pub angle: f64,
    pub scale_x: f64,
    pub scale_y: f64,

    pub pivot_offset_x: f64,
    pub pivot_offset_y: f64,

    pub flip_h: i32,
    pub flip_v: i32,

    pub opacity: f64,

    pub blend_type: BlendType,

    // unused fields
    _vdef_lt_x: i32,
    _vdef_lt_y: i32,
    _vdef_rt_x: i32,
    _vdef_rt_y: i32,
    _vdef_lb_x: i32,
    _vdef_lb_y: i32,
    _vdef_rb_x: i32,
    _vdef_rb_y: i32,
    /*
    _color_label: Option<i32>,
    _blend_color: Option<i32>,
    _vc_lt_rgba: Option<i32>,
    _vc_rt_rgba: Option<i32>,
    _vc_lb_rgba: Option<i32>,
    _vc_rb_rgba: Option<i32>,

    _vp_lt_x: Option<f32>,
    _vp_lt_y: Option<f32>,
    _vp_rt_x: Option<f32>,
    _vp_rt_y: Option<f32>,
    _vp_lb_x: Option<f32>,
    _vp_lb_y: Option<f32>,
    _vp_rb_x: Option<f32>,
    _vp_rb_y: Option<f32>, */
}
