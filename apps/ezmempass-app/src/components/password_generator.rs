/*
 * Password generator component for the EzMemPass application.
 */

use leptos::prelude::*;

/// Password generator component
#[component]
pub fn PasswordGenerator() -> impl IntoView {
    // In a full implementation, these would be reactive signals
    let password = "Correct-horse-battery-staple42";
    let entropy = 60.0;
    let method = "Language Model";
    let memory_aid = "Visualize a correct horse with a battery staple";

    view! {
        <div class="password-generator">
            <div class="password-display">
                <h2>"Generated Password"</h2>
                <div class="password-text">{password}</div>
                <div class="password-info">
                    <div>
                        <span class="info-label">"Entropy: "</span>
                        {format!("{:.1} bits", entropy)}
                    </div>
                    <div>
                        <span class="info-label">"Method: "</span>
                        {method}
                    </div>
                </div>
                <div class="password-memory-aid">
                    <div class="info-label">"Memory Aid:"</div>
                    <div>{memory_aid}</div>
                </div>
                <div class="password-actions">
                    <button class="button primary">"Copy"</button>
                    <button class="button">"Generate New"</button>
                </div>
            </div>
            <div class="generator-options">
                <h2>"Options"</h2>
                <div class="option-group">
                    <div class="option-item">
                        <label for="word-count">"Word Count:"</label>
                        <input
                            type="range"
                            prop:id="word-count"
                            min="3"
                            max="8"
                            value="4"
                        />
                        <span class="option-value">"4"</span>
                    </div>
                    <div class="option-item">
                        <label for="include-uppercase">"Include Uppercase:"</label>
                        <input
                            type="checkbox"
                            prop:id="include-uppercase"
                            checked=true
                        />
                    </div>
                    <div class="option-item">
                        <label for="include-digits">"Include Digits:"</label>
                        <input
                            type="checkbox"
                            prop:id="include-digits"
                            checked=true
                        />
                    </div>
                    <div class="option-item">
                        <label for="include-symbols">"Include Symbols:"</label>
                        <input
                            type="checkbox"
                            prop:id="include-symbols"
                        />
                    </div>
                    <div class="option-item">
                        <label for="generation-method">"Generation Method:"</label>
                        <select prop:id="generation-method">
                            <option value="model" selected=true>"Language Model"</option>
                            <option value="graph">"Graph Search"</option>
                            <option value="random">"Random"</option>
                        </select>
                    </div>
                </div>
            </div>
        </div>
    }
}
