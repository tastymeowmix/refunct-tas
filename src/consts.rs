// FEngineLoop::Tick(FEngineLoop *__hidden this)
// _ZN11FEngineLoop4TickEv
#[cfg(unix)]
pub const FENGINELOOP_TICK_AFTER_UPDATETIME: usize = 0x169DC95;
#[cfg(windows)]
pub const FENGINELOOP_TICK_AFTER_UPDATETIME: usize = 0x4E8BBC;
// FApp::DeltaTime
// static variable inside the binary
#[cfg(unix)]
pub const APP_DELTATIME: usize = 0x51CB4A0;
#[cfg(windows)]
pub const APP_DELTATIME: usize = 0x2239A68;
// FSlateApplication::Tick(FSlateApplication *__hidden this)
// _ZN17FSlateApplication4TickEv
#[cfg(unix)]
pub const FSLATEAPPLICATION_TICK: usize = 0x1A6D0B0;
#[cfg(windows)]
pub const FSLATEAPPLICATION_TICK: usize = 0x730560;
// AMyCharacter::ForcedUnCrouch(AMyCharacter *__hidden this)
// _ZN12AMyCharacter14ForcedUnCrouchEv
#[cfg(unix)]
pub const AMYCHARACTER_EXECFORCEDUNCROUCH: usize = 0x16C6100;
#[cfg(windows)]
pub const AMYCHARACTER_EXECFORCEDUNCROUCH: usize = 0x5005D0;
// FSlateApplication::OnKeyDown(FSlateApplication *this, unsigned int, unsigned int, bool)
// _ZN17FSlateApplication9OnKeyDownEijb
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONKEYDOWN: usize = 0x1A7C860;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONKEYDOWN: usize = 0x721090;
// FSlateApplication::OnKeyUp(FSlateApplication *this, unsigned int, unsigned int, bool)
// _ZN17FSlateApplication7OnKeyUpEijb
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONKEYUP: usize = 0x1A7D880;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONKEYUP: usize = 0x721230;
// FSlateApplication::OnRawMouseMove(FSlateApplication *this, int, int)
// _ZN17FSlateApplication14OnRawMouseMoveEii
#[cfg(unix)]
pub const FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0x1A85900;
#[cfg(windows)]
pub const FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0x721F50;
