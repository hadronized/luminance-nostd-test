//! This module holds all the low-level and platform-specific code to open an OpenGL context and a
//! minimal window.
//!
//! I highly not recommend to dig in too much as it’s very spec / driver code and magic
//! numbers.

#[cfg(target_os="linux")]
mod platform {
  use core::ptr;

  // several hints used to specify the bit depth and double buffering
  const GLX_RGBA: i32 = 4;
  const GLX_DOUBLEBUFFER: i32 = 5;
  const GLX_DEPTH_SIZE: i32 = 12;
  const DOUBLE_BUFF_VISUAL: [i32; 5] = [GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, 0];
  const EXPOSURE_MASK: i64 = 1 << 15;
  const INPUT_OUPUT: u32 = 1;
  const CW_BORDER_PIXEL: u64 = 1 << 3;
  const CW_EVENT_MASK: u64 = 1 << 11;
  const CW_COLORMAP: u64 = 1 << 13;
  const KEY_PRESS_MASK: i64 = 1 << 0;
  const KEY_RELEASE_MASK: i64 = 1 << 1;
  const SUBSTRUCTURE_NOTIFY_MASK: i64 = 1 << 19;
  
  // query structure for visuals
  #[repr(C)]
  struct XVisualInfo {
    visual: *mut Visual,
    visualid: VisualID,
    screen: i32,
    depth: i32,
    c_class: i32,
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    colormap_size: i32,
    bits_per_rgb: i32
  }
  
  type XID = u32;
  
  type VisualID = XID;
  
  enum Visual {} // opaque
  
  type Colormap = XID;
  
  #[repr(C)]
  struct XSetWindowAttributes {
    background_pixmap: Pixmap,
    background_pixel: u32,
    border_pixmap: Pixmap,
    border_pixel: u32,
    bit_gravity: i32,
    win_gravity: i32,
    backing_store: i32,
    backing_planes: u32,
    backing_pixel: u32,
    save_under: Bool,
    event_mask: i64,
    do_not_propagate_mask: i64,
    override_redirect: Bool,
    colormap: Colormap,
    cursor: Cursor
  }
  
  type Pixmap = XID;
  type Bool = i16;
  type Cursor = XID;
  
  enum XExtData {} // opaque
  enum XPrivate {} // opaque
  enum XDisplay {} // opaque
  enum ScreenFormat {} // opaque
  enum XrmHashBucketRec {} // opaque
  
  type XPointer = *mut i8;
  
  struct Display {
    ext_data: *mut XExtData,
    private1: *mut XPrivate,
    fd: i32,
    private2: i32,
    proto_major_version: i32,
    proto_minor_version: i32,
    vendor: *mut i8,
    private3: XID,
    private4: XID,
    private5: XID,
    private6: XID,
    resource_alloc: fn (*mut XDisplay) -> XID,
    byte_order: i32,
    bitmap_unit: i32,
    bitmap_pad: i32,
    bitmap_bit_order: i32,
    nformats: i32,
    pixmap_format: *mut ScreenFormat,
    private8: i32,
    release: i32,
    private9: *mut XPrivate,
    private10: *mut XPrivate,
    qlen: i32,
    last_request_read: u64,
    request: u64,
    private11: XPointer,
    private12: XPointer,
    private13: XPointer,
    private14: XPointer,
    max_request_size: u32,
    db: *mut XrmHashBucketRec,
    private15: fn (*mut XDisplay) -> i32,
    display_name: *mut i8,
    default_screen: i32,
    nscreens: i32,
    screens: *mut Screen,
    motion_buffer: u64,
    private16: u64,
    min_keycode: i32,
    max_keycode: i32,
    private17: XPointer,
    private18: XPointer,
    private19: i32,
    defaults: *mut i8
  }
  
  struct Screen {
    ext_data: *mut XExtData,
    display: *mut Display,
    root: Window,
    width: i32,
    height: i32,
    mwidth: i32,
    mheight: i32,
    ndepths: i32,
    depths: *mut Depth,
    root_depth: i32,
    root_visual: *mut Visual,
    default_gc: GC,
    cmap: Colormap,
    white_pixel: u64,
    black_pixel: u64,
    max_maps: i32,
    min_maps: i32,
    backing_store: i32,
    save_unders: Bool,
    root_input_mask: i64
  }
  
  enum Depth {} // opaque
  
  enum GC_ {} // opaque
  type GC = *mut GC_;
  
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
      _: *const i32
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
  
  unsafe fn open_context(title: &str, width: u32, height: u32) -> Option<()> {
    let c_title = title.as_ptr() as *const i8;
    let argv = [c_title, ptr::null()];
    let pDisp = x_open_display(ptr::null_mut());
  
    if pDisp.is_null() {
      return None;
    }
  
    // make sure GLX is supported
    let glxSupported = glx_query_extension(pDisp, ptr::null_mut(), ptr::null_mut());
    if glxSupported == 0 {
      return None;
    }
  
    let pVI = glx_choose_visual(pDisp, (*pDisp).default_screen, &DOUBLE_BUFF_VISUAL as _);
    
    if pVI.is_null() {
      return None;
    }
  
    let ctx = glx_create_context(pDisp, pVI, ptr::null_mut(), 1);
  
    if ctx.is_null() {
      return None;
    }
  
    let rootwin = (*(*pDisp).screens.offset((*pVI).screen as isize)).root;
    let cmap = x_create_colormap(pDisp, rootwin, (*pVI).visual, 0);
  
    let mut winAttr = XSetWindowAttributes {
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
  
    let win = x_create_window(pDisp, rootwin, 0, 0, width, height, 0, (*pVI).depth, INPUT_OUPUT,
                              (*pVI).visual, CW_BORDER_PIXEL | CW_COLORMAP | CW_EVENT_MASK, &mut winAttr);
  
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
}
