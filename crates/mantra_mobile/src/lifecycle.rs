// ==============================================================================
// mantra_mobile::lifecycle — Cross-Platform Mobile Lifecycle Management
// ==============================================================================

/// Standardized mobile application lifecycle states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppLifecycleState {
    /// Initialized in memory, UI not yet attached
    Created,
    /// App is in foreground, receiving user input and active
    ForegroundActive,
    /// App is in foreground but interrupted (phone call, notification center pulled down)
    ForegroundInactive,
    /// App is obscured, running background tasks or audio
    BackgroundRunning,
    /// App process is frozen in memory by OS (iOS Suspended / Android cached process)
    Suspended,
    /// App process is terminating and resources are being freed
    Destroyed,
}

impl AppLifecycleState {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppLifecycleState::Created => "created",
            AppLifecycleState::ForegroundActive => "active",
            AppLifecycleState::ForegroundInactive => "inactive",
            AppLifecycleState::BackgroundRunning => "background",
            AppLifecycleState::Suspended => "suspended",
            AppLifecycleState::Destroyed => "destroyed",
        }
    }
}

/// Lifecycle event types dispatched by native platforms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleEvent {
    // Android lifecycle equivalents
    AndroidOnCreate,
    AndroidOnStart,
    AndroidOnResume,
    AndroidOnPause,
    AndroidOnStop,
    AndroidOnDestroy,
    AndroidOnLowMemory,

    // iOS lifecycle equivalents
    IosDidFinishLaunching,
    IosDidBecomeActive,
    IosWillResignActive,
    IosDidEnterBackground,
    IosWillEnterForeground,
    IosWillTerminate,
    IosDidReceiveMemoryWarning,
}

/// Lifecycle State Machine tracking mobile state and notifying listeners.
pub struct LifecycleManager {
    current_state: AppLifecycleState,
    history: Vec<AppLifecycleState>,
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LifecycleManager {
    pub fn new() -> Self {
        LifecycleManager {
            current_state: AppLifecycleState::Created,
            history: vec![AppLifecycleState::Created],
        }
    }

    pub fn current_state(&self) -> AppLifecycleState {
        self.current_state
    }

    pub fn transition_to(&mut self, new_state: AppLifecycleState) {
        if self.current_state != new_state {
            self.current_state = new_state;
            self.history.push(new_state);
        }
    }

    /// Processes an Android native lifecycle callback.
    pub fn handle_android_event(&mut self, event: LifecycleEvent) -> AppLifecycleState {
        match event {
            LifecycleEvent::AndroidOnCreate => self.transition_to(AppLifecycleState::Created),
            LifecycleEvent::AndroidOnStart | LifecycleEvent::AndroidOnResume => {
                self.transition_to(AppLifecycleState::ForegroundActive)
            }
            LifecycleEvent::AndroidOnPause => {
                self.transition_to(AppLifecycleState::ForegroundInactive)
            }
            LifecycleEvent::AndroidOnStop => {
                self.transition_to(AppLifecycleState::BackgroundRunning)
            }
            LifecycleEvent::AndroidOnDestroy => {
                self.transition_to(AppLifecycleState::Destroyed)
            }
            _ => {}
        }
        self.current_state
    }

    /// Processes an iOS native lifecycle callback.
    pub fn handle_ios_event(&mut self, event: LifecycleEvent) -> AppLifecycleState {
        match event {
            LifecycleEvent::IosDidFinishLaunching => self.transition_to(AppLifecycleState::Created),
            LifecycleEvent::IosDidBecomeActive => {
                self.transition_to(AppLifecycleState::ForegroundActive)
            }
            LifecycleEvent::IosWillResignActive => {
                self.transition_to(AppLifecycleState::ForegroundInactive)
            }
            LifecycleEvent::IosDidEnterBackground => {
                self.transition_to(AppLifecycleState::BackgroundRunning)
            }
            LifecycleEvent::IosWillEnterForeground => {
                self.transition_to(AppLifecycleState::ForegroundActive)
            }
            LifecycleEvent::IosWillTerminate => {
                self.transition_to(AppLifecycleState::Destroyed)
            }
            _ => {}
        }
        self.current_state
    }

    /// Serializes current state to a JSON event string for WebView listeners.
    pub fn to_event_json(&self) -> String {
        format!(
            r#"{{"event":"lifecycle_change","state":"{}"}}"#,
            self.current_state.as_str()
        )
    }
}

