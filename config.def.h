/* Taken from https://github.com/djpohly/dwl/issues/466 */
#define COLOR(hex)    { ((hex >> 24) & 0xFF) / 255.0f, \
                        ((hex >> 16) & 0xFF) / 255.0f, \
                        ((hex >> 8) & 0xFF) / 255.0f, \
                        (hex & 0xFF) / 255.0f }

/* appearance */
static const int sloppyfocus               = 1;  /* focus follows mouse */
static const int bypass_surface_visibility = 0;  /* 1 means idle inhibitors will disable idle tracking even if it's surface isn't visible  */
static const unsigned int borderpx         = 2;  /* border pixel of windows */
static const float bordercolor[]           = COLOR(0x0a0a0aff);  /* почти чёрный */
static const float focuscolor[]            = COLOR(0x00ff00ff);  /* зелёный */
static const float urgentcolor[]           = COLOR(0xff0000ff);  /* красный для срочных окон */
/* To conform the xdg-protocol, set the alpha to zero to restore the old behavior */
static const float fullscreen_bg[]         = {0.0, 0.0, 0.0, 1.0};  /* чёрный фон в полноэкранном режиме */

/* tagging - TAGCOUNT must be no greater than 31 */
#define TAGCOUNT (5)

/* logging */
static int log_level = WLR_ERROR;

static const Rule rules[] = {
    /* app_id       title     tags mask     isfloating   monitor */
    { "foot",        NULL,     0,            0,           -1 },
    { "Alacritty",   NULL,     0,            0,           -1 },
    { "firefox",     NULL,     1 << 4,       0,           -1 },  /* firefox на 5-м теге */
};

/* layout(s) */
static const Layout layouts[] = {
    /* symbol     arrange function */
    { "[]=",      tile },     /* мозаика */
    { "><>",      NULL },     /* плавающие окна */
    { "[M]",      monocle },  /* одно окно на весь экран */
};

/* monitors */
static const MonitorRule monrules[] = {
    /* defaults */
    { NULL,       0.55, 1,      1,    &layouts[0], WL_OUTPUT_TRANSFORM_NORMAL,   -1,  -1 },
};

/* keyboard */
static const struct xkb_rule_names xkb_rules = {
    .options = NULL,
};

static const int repeat_rate = 25;
static const int repeat_delay = 600;

/* Trackpad */
static const int tap_to_click = 1;
static const int tap_and_drag = 1;
static const int drag_lock = 1;
static const int natural_scrolling = 0;
static const int disable_while_typing = 1;
static const int left_handed = 0;
static const int middle_button_emulation = 0;
static const enum libinput_config_scroll_method scroll_method = LIBINPUT_CONFIG_SCROLL_2FG;
static const enum libinput_config_click_method click_method = LIBINPUT_CONFIG_CLICK_METHOD_BUTTON_AREAS;
static const uint32_t send_events_mode = LIBINPUT_CONFIG_SEND_EVENTS_ENABLED;
static const enum libinput_config_accel_profile accel_profile = LIBINPUT_CONFIG_ACCEL_PROFILE_ADAPTIVE;
static const double accel_speed = 0.0;
static const enum libinput_config_tap_button_map button_map = LIBINPUT_CONFIG_TAP_MAP_LRM;

/* If you want to use the windows key for MODKEY, use WLR_MODIFIER_LOGO */
#define MODKEY WLR_MODIFIER_LOGO

#define TAGKEYS(KEY,SKEY,TAG) \
    { MODKEY,                    KEY,            view,            {.ui = 1 << TAG} }, \
    { MODKEY|WLR_MODIFIER_CTRL,  KEY,            toggleview,      {.ui = 1 << TAG} }, \
    { MODKEY|WLR_MODIFIER_SHIFT, SKEY,           tag,             {.ui = 1 << TAG} }, \
    { MODKEY|WLR_MODIFIER_CTRL|WLR_MODIFIER_SHIFT,SKEY,toggletag, {.ui = 1 << TAG} }

/* helper for spawning shell commands in the pre dwm-5.0 fashion */
#define SHCMD(cmd) { .v = (const char*[]){ "/bin/sh", "-c", cmd, NULL } }

/* commands */
static const char *termcmd[] = { "foot", NULL };
static const char *menucmd[] = { "/home/user/.local/bin/menu.sh", NULL };
static const char *statuscmd[] = { "/home/user/.local/bin/statusbar.sh", NULL };

static const Key keys[] = {
    /* modifier                  key                 function        argument */
    { MODKEY,                    XKB_KEY_Return,     spawn,          {.v = termcmd} },           /* Super+Enter - терминал */
    { MODKEY,                    XKB_KEY_space,      spawn,          {.v = menucmd} },           /* Super+Space - меню системы */
    { MODKEY|WLR_MODIFIER_SHIFT, XKB_KEY_c,          killclient,     {0} },                      /* Super+Shift+c - закрыть окно */
    { MODKEY,                    XKB_KEY_q,          quit,           {0} },                      /* Super+q - выход */
    { MODKEY,                    XKB_KEY_j,          focusstack,     {.i = +1} },                /* Super+j - следующее окно */
    { MODKEY,                    XKB_KEY_k,          focusstack,     {.i = -1} },                /* Super+k - предыдущее окно */
    { MODKEY,                    XKB_KEY_h,          setmfact,       {.f = -0.05} },             /* Super+h - уменьшить окно */
    { MODKEY,                    XKB_KEY_l,          setmfact,       {.f = +0.05} },             /* Super+l - увеличить окно */
    { MODKEY,                    XKB_KEY_Return,     zoom,           {0} },                      /* Super+Enter - развернуть окно */
    { MODKEY,                    XKB_KEY_Tab,        view,           {0} },                      /* Super+Tab - переключить теги */
    { MODKEY,                    XKB_KEY_t,          setlayout,      {.v = &layouts[0]} },       /* Super+t - мозаика */
    { MODKEY,                    XKB_KEY_f,          setlayout,      {.v = &layouts[1]} },       /* Super+f - плавающие окна */
    { MODKEY,                    XKB_KEY_m,          setlayout,      {.v = &layouts[2]} },       /* Super+m - монокль */
    { MODKEY,                    XKB_KEY_space,      setlayout,      {0} },                      /* Super+Space - след. раскладка */
    { MODKEY|WLR_MODIFIER_SHIFT, XKB_KEY_space,      togglefloating, {0} },                      /* Super+Shift+Space - плавающее окно */
    { MODKEY,                    XKB_KEY_e,          togglefullscreen, {0} },                   /* Super+e - полноэкранный режим */
    { MODKEY,                    XKB_KEY_0,          view,           {.ui = ~0} },               /* Super+0 - показать все теги */
    { MODKEY|WLR_MODIFIER_SHIFT, XKB_KEY_0,          tag,            {.ui = ~0} },               /* Super+Shift+0 - переместить окно на все теги */
    { MODKEY,                    XKB_KEY_comma,      focusmon,       {.i = WLR_DIRECTION_LEFT} },  /* Super+, - предыдущий монитор */
    { MODKEY,                    XKB_KEY_period,     focusmon,       {.i = WLR_DIRECTION_RIGHT} }, /* Super+. - следующий монитор */
    
    /* Теги 1-5 */
    TAGKEYS(          XKB_KEY_1, XKB_KEY_exclam,                     0),  /* Super+1 / Super+Shift+! */
    TAGKEYS(          XKB_KEY_2, XKB_KEY_at,                         1),  /* Super+2 / Super+Shift+@ */
    TAGKEYS(          XKB_KEY_3, XKB_KEY_numbersign,                 2),  /* Super+3 / Super+Shift+# */
    TAGKEYS(          XKB_KEY_4, XKB_KEY_dollar,                     3),  /* Super+4 / Super+Shift+$ */
    TAGKEYS(          XKB_KEY_5, XKB_KEY_percent,                    4),  /* Super+5 / Super+Shift+% */
    
    { MODKEY|WLR_MODIFIER_SHIFT, XKB_KEY_Q,          quit,           {0} },  /* Super+Shift+Q - принудительный выход */
};

static const Button buttons[] = {
    { MODKEY, BTN_LEFT,   moveresize,     {.ui = CurMove} },     /* Super+ЛКМ - переместить окно */
    { MODKEY, BTN_MIDDLE, togglefloating, {0} },                 /* Super+Средняя кнопка - плавающее окно */
    { MODKEY, BTN_RIGHT,  moveresize,     {.ui = CurResize} },   /* Super+ПКМ - изменить размер */
};