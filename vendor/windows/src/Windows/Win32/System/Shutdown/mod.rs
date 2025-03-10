#[inline]
pub unsafe fn AbortSystemShutdownA<P0>(lpmachinename: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn AbortSystemShutdownA(lpmachinename : ::windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    AbortSystemShutdownA(lpmachinename.into_param().abi()).ok()
}
#[inline]
pub unsafe fn AbortSystemShutdownW<P0>(lpmachinename: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn AbortSystemShutdownW(lpmachinename : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    AbortSystemShutdownW(lpmachinename.into_param().abi()).ok()
}
#[inline]
pub unsafe fn CheckForHiberboot<P0>(phiberboot: *mut super::super::Foundation::BOOLEAN, bclearflag: P0) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn CheckForHiberboot(phiberboot : *mut super::super::Foundation:: BOOLEAN, bclearflag : super::super::Foundation:: BOOLEAN) -> u32);
    CheckForHiberboot(phiberboot, bclearflag.into_param().abi())
}
#[inline]
pub unsafe fn ExitWindowsEx(uflags: EXIT_WINDOWS_FLAGS, dwreason: SHUTDOWN_REASON) -> ::windows_core::Result<()> {
    ::windows_targets::link!("user32.dll" "system" fn ExitWindowsEx(uflags : EXIT_WINDOWS_FLAGS, dwreason : SHUTDOWN_REASON) -> super::super::Foundation:: BOOL);
    ExitWindowsEx(uflags, dwreason).ok()
}
#[inline]
pub unsafe fn InitiateShutdownA<P0, P1>(lpmachinename: P0, lpmessage: P1, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn InitiateShutdownA(lpmachinename : ::windows_core::PCSTR, lpmessage : ::windows_core::PCSTR, dwgraceperiod : u32, dwshutdownflags : SHUTDOWN_FLAGS, dwreason : SHUTDOWN_REASON) -> u32);
    InitiateShutdownA(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), dwgraceperiod, dwshutdownflags, dwreason)
}
#[inline]
pub unsafe fn InitiateShutdownW<P0, P1>(lpmachinename: P0, lpmessage: P1, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn InitiateShutdownW(lpmachinename : ::windows_core::PCWSTR, lpmessage : ::windows_core::PCWSTR, dwgraceperiod : u32, dwshutdownflags : SHUTDOWN_FLAGS, dwreason : SHUTDOWN_REASON) -> u32);
    InitiateShutdownW(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), dwgraceperiod, dwshutdownflags, dwreason)
}
#[inline]
pub unsafe fn InitiateSystemShutdownA<P0, P1, P2, P3>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: P2, brebootaftershutdown: P3) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn InitiateSystemShutdownA(lpmachinename : ::windows_core::PCSTR, lpmessage : ::windows_core::PCSTR, dwtimeout : u32, bforceappsclosed : super::super::Foundation:: BOOL, brebootaftershutdown : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    InitiateSystemShutdownA(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), dwtimeout, bforceappsclosed.into_param().abi(), brebootaftershutdown.into_param().abi()).ok()
}
#[inline]
pub unsafe fn InitiateSystemShutdownExA<P0, P1, P2, P3>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: P2, brebootaftershutdown: P3, dwreason: SHUTDOWN_REASON) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn InitiateSystemShutdownExA(lpmachinename : ::windows_core::PCSTR, lpmessage : ::windows_core::PCSTR, dwtimeout : u32, bforceappsclosed : super::super::Foundation:: BOOL, brebootaftershutdown : super::super::Foundation:: BOOL, dwreason : SHUTDOWN_REASON) -> super::super::Foundation:: BOOL);
    InitiateSystemShutdownExA(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), dwtimeout, bforceappsclosed.into_param().abi(), brebootaftershutdown.into_param().abi(), dwreason).ok()
}
#[inline]
pub unsafe fn InitiateSystemShutdownExW<P0, P1, P2, P3>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: P2, brebootaftershutdown: P3, dwreason: SHUTDOWN_REASON) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn InitiateSystemShutdownExW(lpmachinename : ::windows_core::PCWSTR, lpmessage : ::windows_core::PCWSTR, dwtimeout : u32, bforceappsclosed : super::super::Foundation:: BOOL, brebootaftershutdown : super::super::Foundation:: BOOL, dwreason : SHUTDOWN_REASON) -> super::super::Foundation:: BOOL);
    InitiateSystemShutdownExW(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), dwtimeout, bforceappsclosed.into_param().abi(), brebootaftershutdown.into_param().abi(), dwreason).ok()
}
#[inline]
pub unsafe fn InitiateSystemShutdownW<P0, P1, P2, P3>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: P2, brebootaftershutdown: P3) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn InitiateSystemShutdownW(lpmachinename : ::windows_core::PCWSTR, lpmessage : ::windows_core::PCWSTR, dwtimeout : u32, bforceappsclosed : super::super::Foundation:: BOOL, brebootaftershutdown : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    InitiateSystemShutdownW(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), dwtimeout, bforceappsclosed.into_param().abi(), brebootaftershutdown.into_param().abi()).ok()
}
#[inline]
pub unsafe fn LockWorkStation() -> ::windows_core::Result<()> {
    ::windows_targets::link!("user32.dll" "system" fn LockWorkStation() -> super::super::Foundation:: BOOL);
    LockWorkStation().ok()
}
#[inline]
pub unsafe fn ShutdownBlockReasonCreate<P0, P1>(hwnd: P0, pwszreason: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("user32.dll" "system" fn ShutdownBlockReasonCreate(hwnd : super::super::Foundation:: HWND, pwszreason : ::windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    ShutdownBlockReasonCreate(hwnd.into_param().abi(), pwszreason.into_param().abi()).ok()
}
#[inline]
pub unsafe fn ShutdownBlockReasonDestroy<P0>(hwnd: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("user32.dll" "system" fn ShutdownBlockReasonDestroy(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    ShutdownBlockReasonDestroy(hwnd.into_param().abi()).ok()
}
#[inline]
pub unsafe fn ShutdownBlockReasonQuery<P0>(hwnd: P0, pwszbuff: ::windows_core::PWSTR, pcchbuff: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link!("user32.dll" "system" fn ShutdownBlockReasonQuery(hwnd : super::super::Foundation:: HWND, pwszbuff : ::windows_core::PWSTR, pcchbuff : *mut u32) -> super::super::Foundation:: BOOL);
    ShutdownBlockReasonQuery(hwnd.into_param().abi(), ::core::mem::transmute(pwszbuff), pcchbuff).ok()
}
pub const EWX_ARSO: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(67108864u32);
pub const EWX_BOOTOPTIONS: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(16777216u32);
pub const EWX_CHECK_SAFE_FOR_SERVER: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(134217728u32);
pub const EWX_FORCE: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(4u32);
pub const EWX_FORCEIFHUNG: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(16u32);
pub const EWX_HYBRID_SHUTDOWN: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(4194304u32);
pub const EWX_LOGOFF: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(0u32);
pub const EWX_POWEROFF: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(8u32);
pub const EWX_QUICKRESOLVE: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(32u32);
pub const EWX_REBOOT: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(2u32);
pub const EWX_RESTARTAPPS: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(64u32);
pub const EWX_SHUTDOWN: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(1u32);
pub const EWX_SYSTEM_INITIATED: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(268435456u32);
pub const MAX_NUM_REASONS: u32 = 256u32;
pub const MAX_REASON_BUGID_LEN: u32 = 32u32;
pub const MAX_REASON_COMMENT_LEN: u32 = 512u32;
pub const MAX_REASON_DESC_LEN: u32 = 256u32;
pub const MAX_REASON_NAME_LEN: u32 = 64u32;
pub const POLICY_SHOWREASONUI_ALWAYS: u32 = 1u32;
pub const POLICY_SHOWREASONUI_NEVER: u32 = 0u32;
pub const POLICY_SHOWREASONUI_SERVERONLY: u32 = 3u32;
pub const POLICY_SHOWREASONUI_WORKSTATIONONLY: u32 = 2u32;
pub const SHTDN_REASON_FLAG_CLEAN_UI: SHUTDOWN_REASON = SHUTDOWN_REASON(67108864u32);
pub const SHTDN_REASON_FLAG_COMMENT_REQUIRED: SHUTDOWN_REASON = SHUTDOWN_REASON(16777216u32);
pub const SHTDN_REASON_FLAG_DIRTY_PROBLEM_ID_REQUIRED: SHUTDOWN_REASON = SHUTDOWN_REASON(33554432u32);
pub const SHTDN_REASON_FLAG_DIRTY_UI: SHUTDOWN_REASON = SHUTDOWN_REASON(134217728u32);
pub const SHTDN_REASON_FLAG_MOBILE_UI_RESERVED: SHUTDOWN_REASON = SHUTDOWN_REASON(268435456u32);
pub const SHTDN_REASON_FLAG_PLANNED: SHUTDOWN_REASON = SHUTDOWN_REASON(2147483648u32);
pub const SHTDN_REASON_FLAG_USER_DEFINED: SHUTDOWN_REASON = SHUTDOWN_REASON(1073741824u32);
pub const SHTDN_REASON_LEGACY_API: SHUTDOWN_REASON = SHUTDOWN_REASON(2147942400u32);
pub const SHTDN_REASON_MAJOR_APPLICATION: SHUTDOWN_REASON = SHUTDOWN_REASON(262144u32);
pub const SHTDN_REASON_MAJOR_HARDWARE: SHUTDOWN_REASON = SHUTDOWN_REASON(65536u32);
pub const SHTDN_REASON_MAJOR_LEGACY_API: SHUTDOWN_REASON = SHUTDOWN_REASON(458752u32);
pub const SHTDN_REASON_MAJOR_NONE: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
pub const SHTDN_REASON_MAJOR_OPERATINGSYSTEM: SHUTDOWN_REASON = SHUTDOWN_REASON(131072u32);
pub const SHTDN_REASON_MAJOR_OTHER: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
pub const SHTDN_REASON_MAJOR_POWER: SHUTDOWN_REASON = SHUTDOWN_REASON(393216u32);
pub const SHTDN_REASON_MAJOR_SOFTWARE: SHUTDOWN_REASON = SHUTDOWN_REASON(196608u32);
pub const SHTDN_REASON_MAJOR_SYSTEM: SHUTDOWN_REASON = SHUTDOWN_REASON(327680u32);
pub const SHTDN_REASON_MINOR_BLUESCREEN: SHUTDOWN_REASON = SHUTDOWN_REASON(15u32);
pub const SHTDN_REASON_MINOR_CORDUNPLUGGED: SHUTDOWN_REASON = SHUTDOWN_REASON(11u32);
pub const SHTDN_REASON_MINOR_DC_DEMOTION: SHUTDOWN_REASON = SHUTDOWN_REASON(34u32);
pub const SHTDN_REASON_MINOR_DC_PROMOTION: SHUTDOWN_REASON = SHUTDOWN_REASON(33u32);
pub const SHTDN_REASON_MINOR_DISK: SHUTDOWN_REASON = SHUTDOWN_REASON(7u32);
pub const SHTDN_REASON_MINOR_ENVIRONMENT: SHUTDOWN_REASON = SHUTDOWN_REASON(12u32);
pub const SHTDN_REASON_MINOR_HARDWARE_DRIVER: SHUTDOWN_REASON = SHUTDOWN_REASON(13u32);
pub const SHTDN_REASON_MINOR_HOTFIX: SHUTDOWN_REASON = SHUTDOWN_REASON(17u32);
pub const SHTDN_REASON_MINOR_HOTFIX_UNINSTALL: SHUTDOWN_REASON = SHUTDOWN_REASON(23u32);
pub const SHTDN_REASON_MINOR_HUNG: SHUTDOWN_REASON = SHUTDOWN_REASON(5u32);
pub const SHTDN_REASON_MINOR_INSTALLATION: SHUTDOWN_REASON = SHUTDOWN_REASON(2u32);
pub const SHTDN_REASON_MINOR_MAINTENANCE: SHUTDOWN_REASON = SHUTDOWN_REASON(1u32);
pub const SHTDN_REASON_MINOR_MMC: SHUTDOWN_REASON = SHUTDOWN_REASON(25u32);
pub const SHTDN_REASON_MINOR_NETWORKCARD: SHUTDOWN_REASON = SHUTDOWN_REASON(9u32);
pub const SHTDN_REASON_MINOR_NETWORK_CONNECTIVITY: SHUTDOWN_REASON = SHUTDOWN_REASON(20u32);
pub const SHTDN_REASON_MINOR_NONE: SHUTDOWN_REASON = SHUTDOWN_REASON(255u32);
pub const SHTDN_REASON_MINOR_OTHER: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
pub const SHTDN_REASON_MINOR_OTHERDRIVER: SHUTDOWN_REASON = SHUTDOWN_REASON(14u32);
pub const SHTDN_REASON_MINOR_POWER_SUPPLY: SHUTDOWN_REASON = SHUTDOWN_REASON(10u32);
pub const SHTDN_REASON_MINOR_PROCESSOR: SHUTDOWN_REASON = SHUTDOWN_REASON(8u32);
pub const SHTDN_REASON_MINOR_RECONFIG: SHUTDOWN_REASON = SHUTDOWN_REASON(4u32);
pub const SHTDN_REASON_MINOR_SECURITY: SHUTDOWN_REASON = SHUTDOWN_REASON(19u32);
pub const SHTDN_REASON_MINOR_SECURITYFIX: SHUTDOWN_REASON = SHUTDOWN_REASON(18u32);
pub const SHTDN_REASON_MINOR_SECURITYFIX_UNINSTALL: SHUTDOWN_REASON = SHUTDOWN_REASON(24u32);
pub const SHTDN_REASON_MINOR_SERVICEPACK: SHUTDOWN_REASON = SHUTDOWN_REASON(16u32);
pub const SHTDN_REASON_MINOR_SERVICEPACK_UNINSTALL: SHUTDOWN_REASON = SHUTDOWN_REASON(22u32);
pub const SHTDN_REASON_MINOR_SYSTEMRESTORE: SHUTDOWN_REASON = SHUTDOWN_REASON(26u32);
pub const SHTDN_REASON_MINOR_TERMSRV: SHUTDOWN_REASON = SHUTDOWN_REASON(32u32);
pub const SHTDN_REASON_MINOR_UNSTABLE: SHUTDOWN_REASON = SHUTDOWN_REASON(6u32);
pub const SHTDN_REASON_MINOR_UPGRADE: SHUTDOWN_REASON = SHUTDOWN_REASON(3u32);
pub const SHTDN_REASON_MINOR_WMI: SHUTDOWN_REASON = SHUTDOWN_REASON(21u32);
pub const SHTDN_REASON_NONE: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
pub const SHTDN_REASON_UNKNOWN: SHUTDOWN_REASON = SHUTDOWN_REASON(255u32);
pub const SHTDN_REASON_VALID_BIT_MASK: SHUTDOWN_REASON = SHUTDOWN_REASON(3238002687u32);
pub const SHUTDOWN_ARSO: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(8192u32);
pub const SHUTDOWN_CHECK_SAFE_FOR_SERVER: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(16384u32);
pub const SHUTDOWN_FORCE_OTHERS: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(1u32);
pub const SHUTDOWN_FORCE_SELF: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(2u32);
pub const SHUTDOWN_GRACE_OVERRIDE: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(32u32);
pub const SHUTDOWN_HYBRID: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(512u32);
pub const SHUTDOWN_INSTALL_UPDATES: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(64u32);
pub const SHUTDOWN_MOBILE_UI: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(4096u32);
pub const SHUTDOWN_NOREBOOT: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(16u32);
pub const SHUTDOWN_POWEROFF: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(8u32);
pub const SHUTDOWN_RESTART: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(4u32);
pub const SHUTDOWN_RESTARTAPPS: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(128u32);
pub const SHUTDOWN_RESTART_BOOTOPTIONS: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(1024u32);
pub const SHUTDOWN_SKIP_SVC_PRESHUTDOWN: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(256u32);
pub const SHUTDOWN_SOFT_REBOOT: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(2048u32);
pub const SHUTDOWN_SYSTEM_INITIATED: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(65536u32);
pub const SHUTDOWN_TYPE_LEN: u32 = 32u32;
pub const SHUTDOWN_VAIL_CONTAINER: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(32768u32);
pub const SNAPSHOT_POLICY_ALWAYS: u32 = 1u32;
pub const SNAPSHOT_POLICY_NEVER: u32 = 0u32;
pub const SNAPSHOT_POLICY_UNPLANNED: u32 = 2u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct EXIT_WINDOWS_FLAGS(pub u32);
impl ::windows_core::TypeKind for EXIT_WINDOWS_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EXIT_WINDOWS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXIT_WINDOWS_FLAGS").field(&self.0).finish()
    }
}
impl EXIT_WINDOWS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for EXIT_WINDOWS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EXIT_WINDOWS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EXIT_WINDOWS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EXIT_WINDOWS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EXIT_WINDOWS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SHUTDOWN_FLAGS(pub u32);
impl ::windows_core::TypeKind for SHUTDOWN_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SHUTDOWN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHUTDOWN_FLAGS").field(&self.0).finish()
    }
}
impl SHUTDOWN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SHUTDOWN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHUTDOWN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHUTDOWN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHUTDOWN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHUTDOWN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SHUTDOWN_REASON(pub u32);
impl ::windows_core::TypeKind for SHUTDOWN_REASON {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SHUTDOWN_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHUTDOWN_REASON").field(&self.0).finish()
    }
}
impl SHUTDOWN_REASON {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SHUTDOWN_REASON {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHUTDOWN_REASON {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHUTDOWN_REASON {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHUTDOWN_REASON {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHUTDOWN_REASON {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
