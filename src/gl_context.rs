//! This module holds all the low-level and platform-specific code to open an OpenGL context and a
//! minimal window.
//!
//! I highly not recommend to dig in too much as it’s very spec / driver code and magic
//! numbers.

// several hints used to specify the bit depth and double buffering
const GLX_RGBA: i32 = 4;
const GLX_DOUBLEBUFFER: i32 = 5;
const GLX_DEPTH_SIZE: i32 = 12;
const DOUBLE_BUFF_VISUAL: [i32; 5] = [GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, 0];

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
type Visual = (); // opaque

/*

 char const TITLE[] = "Lightning Road To Liquid Radiator";
  int /* const */ DOUBLE_BUFF_VISUAL[] = {
      GLX_RGBA
    , GLX_DEPTH_SIZE, 24
    , GLX_DOUBLEBUFFER
    , None
  };
//    {'r','g','b','a','_','i','n','t','r','o',0}
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
