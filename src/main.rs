use candelabre_windowing::{
    CandlDimension, CandlNoState, CandlOptions,
    CandlRenderer, CandlSurfaceBuilder, CandlWindow
};
use gl;
use glutin::event::{
    ElementState, Event, KeyboardInput, StartCause,
    VirtualKeyCode, WindowEvent
};
use glutin::event_loop::{ControlFlow, EventLoop};
use nvg_gl::Renderer;
use nvg::{Color, Context, Extent, Point};

mod ui;

struct Drawer {
    context: Option<Context<Renderer>>,
    factor: f64,
    size: (u32, u32)
}

impl CandlRenderer<Drawer, CandlNoState, ()> for Drawer {
    fn init() -> Self {
        Self {
            context: None,
            factor: 0.0,
            size: (0, 0)
        }
    }

    fn finalize(&mut self) {
        //
        let renderer = Renderer::create().unwrap();
        let mut context = Context::create(renderer).unwrap();
        let text_font = include_bytes!("../resources/Berylium.ttf").to_vec();
        context.create_font("berylium", text_font).unwrap();
        //
        let icon_font = include_bytes!("../resources/la-regular-400.ttf").to_vec();
        context.create_font("icons", icon_font).unwrap();
        //
        // TODO : add la font
        //
        self.context = Some(context);
        //
    }

    fn set_scale_factor(&mut self, factor: f64) { self.factor = factor; }

    fn set_size(&mut self, size: (u32, u32)) {
        self.size = size;
        let (w, h) = size;
        unsafe { gl::Viewport(0, 0, w as i32, h as i32); }
    }

    fn draw_frame(&mut self, _: &CandlNoState) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(
                gl::COLOR_BUFFER_BIT |
                gl::DEPTH_BUFFER_BIT |
                gl::STENCIL_BUFFER_BIT
            );
        }
        let (w, h) = self.size;
        if let Some(ctx) = &mut self.context {
            ctx.begin_frame(Extent::new(w as f32, h as f32), self.factor as f32).unwrap();
            ctx.save();
            //
            ctx.font_size(50.0);
            //
            ctx.font("berylium");
            //
            ctx.text((150, 80), "test").unwrap();
            //
            //
            ctx.font("icons");
            //
            ctx.text((150, 200), "\u{f0eb}").unwrap();
            //
            ctx.fill_paint(Color::rgb_i(255, 100, 0));
            //
            //
            ctx.fill().unwrap();
            //
            //
            ctx.restore();
            ctx.end_frame().unwrap();
        }
    }
}

fn main() {
    let el = EventLoop::new();
    let mut surface = CandlSurfaceBuilder::new()
        .dim(CandlDimension::Classic(1000, 600))
        .title("PoC - Quarks")
        .options(CandlOptions::default())
        .render(Drawer::init())
        .no_state()
        .video_mode(el.primary_monitor().video_modes().next().unwrap())
        .build(&el)
        .unwrap();
    //
    // reading at zmain.c, line 906
    // struct context *ctx;
    //
    // struct context => qk.h, line 565
    //
    //
    // TODO : create a window
    //
    //
    // struct config cfg; qk.h, line 561
    //
    //
    el.run(move |evt, _, ctrl_flow| {
        match evt {
            Event::LoopDestroyed => return,
            Event::NewEvents(StartCause::Init) =>
                *ctrl_flow = ControlFlow::Wait,
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::Resized(nsize) => surface.resize(nsize),
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    }, ..
                } => *ctrl_flow = ControlFlow::Exit,
                //
                //
                //
                _ => ()
            }
            Event::RedrawEventsCleared =>
                if surface.check_redraw() { surface.request_redraw(); },
            Event::RedrawRequested(_) =>
                surface.draw(),
            _ => ()
        }
    });
}
