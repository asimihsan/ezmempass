/*
 * Settings page component for the EzMemPass application.
 */

use leptos::prelude::*;

/// Settings page component
#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Settings"</h1>
            <div class="card">
                <h2>"Password Generation Options"</h2>
                <div class="settings-group">
                    <div class="setting-item">
                        <span>"Word Count"</span>
                        <input type="range" min="3" max="8" value="4" />
                    </div>
                    <div class="setting-item">
                        <span>"Include Uppercase"</span>
                        <input type="checkbox" checked=true />
                    </div>
                    <div class="setting-item">
                        <span>"Include Digits"</span>
                        <input type="checkbox" checked=true />
                    </div>
                    <div class="setting-item">
                        <span>"Include Symbols"</span>
                        <input type="checkbox" />
                    </div>
                </div>
                <h2>"Language Options"</h2>
                <div class="settings-group">
                    <div class="setting-item">
                        <span>"Language"</span>
                        <select>
                            <option value="en" selected=true>"English"</option>
                            <option value="es">"Spanish"</option>
                            <option value="fr">"French"</option>
                            <option value="de">"German"</option>
                        </select>
                    </div>
                </div>
                <button>"Save Settings"</button>
            </div>
        </div>
    }
}
