import { MEDIA_UI } from "@utils/path";
import { LitElement, html, css, customElement, property } from "lit-element";
import "@elements/entry/user/_common/auth-page";

const STR_TITLE = "Sign Up - Step 1";

@customElement("page-register-step1")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                .inside-wrapper {
                    width: 624px;
                }
                h1 {
                    font-size: 32px;
                    font-weight: 900;
                    color: #5662a3;
                }
                ::slotted([slot="topleft"]),
                ::slotted([slot="topright"]),
                ::slotted([slot="bottomleft"]),
                ::slotted([slot="bottomright"]) {
                    max-width: 296px;
                    min-width: 296px;
                    margin-bottom: 40px;
                }
                ::slotted([slot="topleft"]),
                ::slotted([slot="bottomleft"]) {
                    margin-right: 32px;
                }
                ::slotted([slot="input"]) {
                    margin-top: 20px;
                }
                ::slotted([slot="passwordreminder"]) {
                    text-align: end;
                }
                ::slotted([slot="subtitle"]) {
                    white-space: nowrap;
                }
                ::slotted([slot="submit"]) {
                    margin-top: 40px;
                }
                ::slotted([slot="location"]) {
                    margin-bottom: 40px;
                }
                ::slotted([slot="noaccount"]:last-child) {
                    margin-left: 4px;
                }
                .account-wrapper {
                    display: flex;
                    align-items: center;
                    margin-top: 16px;
                }
                .two-row {
                    display: flex;
                }

                .spacer {
                    height: 20px;
                }
                .hidden {
                    display: none;
                }
                .password-wrapper {
                    position: relative;
                }
                .password-wrapper div {
                    position: absolute;
                    top: 33%;
                    right: -76px;
                }
            `,
        ];
    }

    render() {
        return html`
            <auth-page img="entry/user/side/step-1.webp">
                <h1>${STR_TITLE}</h1>
                <div class="inside-wrapper">
                    <div class="two-row">
                        <slot name="topleft"></slot>
                        <slot name="topright"></slot>
                    </div>
                    <div class="two-row">
                        <slot name="bottomleft"></slot>
                        <slot name="bottomright"></slot>
                    </div>
                    <slot name="location"></slot>
                    <slot name="username"></slot>
                    <div class="spacer"></div>
                    <div class="password-wrapper">
                        <slot name="checkbox"> </slot>

                        <p></p>
                        <slot name="submit"></slot>
                    </div>
                    <slot name="footer"></slot>
                </div>
            </auth-page>
        `;
    }
}
