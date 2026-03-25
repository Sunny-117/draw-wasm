use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// 渲染DOM tree的时候会运行此函数
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    // 获取设备像素比，解决 HiDPI/Retina 屏幕下线条模糊的问题
    let dpr = window.device_pixel_ratio();
    let css_width = 700u32;
    let css_height = 600u32;

    document.body().unwrap().append_child(&canvas)?;
    // canvas 缓冲区按 dpr 放大，确保物理像素 1:1 对应
    canvas.set_width((css_width as f64 * dpr) as u32);
    canvas.set_height((css_height as f64 * dpr) as u32);
    // CSS 样式固定显示尺寸
    let style = canvas.style();
    style.set_property("width", &format!("{}px", css_width))?;
    style.set_property("height", &format!("{}px", css_height))?;
    style.set_property("border", "solid")?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    // 缩放绘图上下文，使逻辑坐标与 CSS 像素对齐
    context.scale(dpr, dpr)?;
    // 设置线条样式，让笔画更平滑清晰
    context.set_line_cap("round");
    context.set_line_join("round");
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget(); // 关闭闭包
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget(); // 关闭闭包
    }

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget(); // 关闭闭包
    }

    Ok(())
}
