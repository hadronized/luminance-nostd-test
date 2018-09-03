//! This module holds all the low-level and platform-specific code to open an OpenGL context and a
//! minimal window.
//!
//! I highly not recommend to dig in too much as it’s very spec / driver code and magic
//! numbers.

use core::ptr;

// several hints used to specify the bit depth and double buffering
const GLX_RGBA: i32 = 4;
const GLX_DOUBLEBUFFER: i32 = 5;
const GLX_DEPTH_SIZE: i32 = 12;
const DOUBLE_BUFF_VISUAL: [i32; 5] = [GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, 0];
const EXPOSURE_MASK: i32 = 1 << 15;
const INPUT_OUPUT: u16 = 1;
const CW_BORDER_PIXEL: u32 = 1 << 3;
const CW_EVENT_MASK: u32 = 1 << 11;
const CW_COLORMAP: u32 = 1 << 13;
const KEY_PRESS_MASK: i32 = 1 << 0;
const KEY_RELEASE_MASK: i32 = 1 << 1;
const SUBSTRUCTURE_NOTIFY_MASK: i32 = 1 << 19;

// query structure for visuals
#[repr(C)]
struct XVisualInfo {
  visual: *mut Visual,
  visualid: VisualID,
  screen: i16,
  depth: i16,
  c_class: i16,
  red_mask: u32,
  green_mask: u32,
  blue_mask: u32,
  colormap_size: i16,
  bits_per_rgb: i16
}

type VisualID = u32;

enum Visual {} // opaque

type Colormap = u32;

#[repr(C)]
struct XSetWindowAttributes {
  background_pixmap: Pixmap,
  background_pixel: u32,
  border_pixmap: Pixmap,
  border_pixel: u32,
  bit_gravity: i16,
  win_gravity: i16,
  backing_store: i16,
  backing_planes: u32,
  backing_pixel: u32,
  save_under: Bool,
  event_mask: i32,
  do_not_propagate_mask: i32,
  override_redirect: Bool,
  colormap: Colormap,
  cursor: Cursor
}

type Pixmap = u32;
type Bool = i16;
type Cursor = u32;

enum Display {} // opaque

enum GLcontextRec {} // opaque
type GLXContext = *mut GLcontextRec;

type Window = u32;

enum XSizeHints {} // opaque

type GLXDrawable = u32;

type Drawable = u32;

#[repr(C)]
struct XColor {
  pixel: u32,
  red: u16,
  green: u16,
  blue: u16,
  flags: i8,
  pad: i8
}

// functions
extern "system" {
  // Xlib
  #[link_name = "XOpenDisplay"] fn x_open_display(
    _: *const i8
  ) -> *mut Display;

  #[link_name = "XCreateColormap"] fn x_create_colormap(
    _: *mut Display,
    _: Window,
    _: *mut Visual,
    _: i32
  ) -> Colormap;

  #[link_name = "XCreateWindow"] fn x_create_window(
    _: *mut Display,
    _: Window,
    _: i32,
    _: i32,
    _: u32,
    _: u32,
    _: u32,
    _: i32,
    _: u32,
    _: *mut Visual,
    _: u64,
    _: *mut XSetWindowAttributes
  ) -> Window;

  #[link_name = "XSetStandardProperties"] fn x_set_standard_properties(
    _: *mut Display,
    _: Window,
    _: *const i8,
    _: *const i8,
    _: Pixmap,
    _: *mut *mut i8,
    _: i32,
    _: *mut XSizeHints
  ) -> i32;

  #[link_name = "XMapWindow"] fn x_map_window(
    _: *mut Display,
    _: Window
  ) -> i32;

  #[link_name = "XCreateBitmapFromData"] fn x_create_bitmap_from_data(
    _: *mut Display,
    _: Drawable,
    _: *const i8,
    _: u32,
    _: u32
  ) -> Pixmap;

  #[link_name = "XCreatePixmapCursor"] fn x_create_pixmap_cursor(
    _: *mut Display,
    _: Pixmap,
    _: Pixmap,
    _: *mut XColor,
    _: *mut XColor,
    _: u32,
    _: u32
  ) -> Cursor;

  #[link_name = "XDefineCursor"] fn x_define_cursor(
    _: *mut Display,
    _: Window,
    _: Cursor
  ) -> i32;

  #[link_name = "XFreeCursor"] fn x_free_cursor(
    _: *mut Display,
    _: Cursor
  ) -> i32;

  #[link_name = "XFreePixmap"] fn x_free_pixmap(
    _: *mut Display,
    _: Pixmap
  ) -> i32;

  #[link_name = "XDestroyWindow"] fn x_destroy_window(
    _: *mut Display,
    _: Window
  ) -> i32;

  #[link_name = "XCloseDisplay"] fn x_close_display(
    _: *mut Display
  ) -> i32;

  // GLX
  #[link_name = "glXQueryExtension"] fn glx_query_extension(
    _: *mut Display,
    _: *mut i32,
    _: *mut i32
  ) -> Bool;

  #[link_name = "glXChooseVisual"] fn glx_choose_visual(
    _: *mut Display,
    _: i32,
    _: *mut i32
  ) -> *mut XVisualInfo;

  #[link_name = "glXCreateContext"] fn glx_create_context(
    _: *mut Display,
    _: *mut XVisualInfo,
    _: GLXContext,
    _: Bool
  ) -> GLXContext;

  #[link_name = "glXMakeCurrent"] fn glx_make_current(
    _: *mut Display,
    _: GLXDrawable,
    _: GLXContext
  );

  #[link_name = "glXSwapBuffers"] fn glx_swap_buffers(
    _: *mut Display,
    _: GLXDrawable
  );
}

fn open_context(title: &str) -> Option<()> {
  let c_title = title.as_ptr() as *const i8;
  let cmap = 0;
  let winAttr = XSetWindowAttributes {
    // interesting values
    colormap: cmap,
    event_mask: EXPOSURE_MASK | KEY_PRESS_MASK | KEY_RELEASE_MASK | SUBSTRUCTURE_NOTIFY_MASK,
    // no one gives a shit
    background_pixmap: 0,
    background_pixel: 0,
    border_pixmap: 0,
    border_pixel: 0,
    bit_gravity: 0,
    win_gravity: 0,
    backing_store: 0,
    backing_planes: 0,
    backing_pixel: 0,
    save_under: 0,
    do_not_propagate_mask: 0,
    override_redirect: 0,
    cursor: 0,
  };
  let argv = [c_title, ptr::null()];
  let pDisp = unsafe { x_open_display(ptr::null_mut()) };

  if pDisp.is_null() {
    return None;
  }

  // make sure GLX is supported
  let glxSupported = unsafe { glx_query_extension(pDisp, ptr::null_mut(), ptr::null_mut()) };
  if glxSupported == 0 {
    return None;
  }

  let pVI = glx_choose_visual(pDisp, 

  Some(())
}

/*

 char const TITLE[] = "Lightning Road To Liquid Radiator";
}
window_c::window_c(unsigned width, unsigned height, bool full) {
  XVisualInfo *pVI;
  Colormap cmap;
  XSetWindowAttributes winAttr;
  char const *argv[] = { TITLE, 0 };

  _pDisp = XOpenDisplay(nullptr);
  if (!_pDisp)
    throw std::runtime_error("X failed to open display");

  /* make sure GLX is supported */
  if (!glXQueryExtension(_pDisp, nullptr, nullptr))
    throw std::runtime_error("GLX not supported");

  pVI = glXChooseVisual(_pDisp, DefaultScreen(_pDisp), DOUBLE_BUFF_VISUAL);
  if (!pVI)
    throw std::runtime_error("GLX failed to choose the visual");

  _cntx = glXCreateContext(_pDisp, pVI, nullptr, GL_TRUE);
  if (!_cntx)
    throw std::runtime_error("GLX failed to create the OpenGL context");

  auto rootwin = RootWindow(_pDisp, pVI->screen);
  cmap = XCreateColormap(_pDisp, rootwin, pVI->visual, AllocNone);

  winAttr.colormap     = cmap;
  winAttr.border_pixel = 0;
  winAttr.event_mask   = ExposureMask           |
    KeyPressMask           |
    KeyReleaseMask         |
    SubstructureNotifyMask;

  _win = XCreateWindow(_pDisp, rootwin, 0, 0, width, height, 0, pVI->depth, InputOutput, pVI->visual, 
                       CWBorderPixel | CWColormap | CWEventMask, &winAttr );
  if (!_win)
    throw std::runtime_error("X failed to create the window");

  char wclss[] = { 0, 0, 0, full, 'r', 'g', 'b', 'a', '_', 'i', 'n', 't', 'r', 'o', 0 };
  XSetStandardProperties(_pDisp, _win, wclss, wclss, None, const_cast<char**>(argv), 1, NULL);
  glXMakeCurrent(_pDisp, _win, _cntx);

  XMapWindow(_pDisp, _win);

  /* hide the cursor */
  Cursor invisibleCursor;
  Pixmap bitmapNoData;
  XColor black;
  static char noData[] = { 0,0,0,0,0,0,0,0 };
  black.red = black.green = black.blue = 0;

  bitmapNoData = XCreateBitmapFromData(_pDisp, _win, noData, 8, 8);
  invisibleCursor = XCreatePixmapCursor(_pDisp, bitmapNoData, bitmapNoData, 
      &black, &black, 0, 0);
  XDefineCursor(_pDisp, _win, invisibleCursor);
  XFreeCursor(_pDisp, invisibleCursor);
  XFreePixmap(_pDisp, bitmapNoData);
}

window_c::~window_c() {
  XDestroyWindow(_pDisp, _win);
  XCloseDisplay(_pDisp);
}

void window_c::swap_buffers() {
  glXSwapBuffers(_pDisp, _win);
}

Display * window_c::display() {
  return _pDisp;
}

*/
