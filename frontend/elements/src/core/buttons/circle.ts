import { LitElement, html, css, customElement, property } from "lit-element";
import { colorStyles, colorValues } from "@elements/_styles/colors";

@customElement("button-circle")
export class CircleButton extends LitElement {
  static get styles() {
    return [
      colorStyles,
      css`
        :host([color=green]) {
          --color: var(--dark-green-1);
        }
        :host([color=blue]) {
          --color: var(--main-blue);
        }
        main {
          cursor: pointer;
          display: flex;
          flex-direction: column;
          align-items: center;

          /* if you change this, also need to change step-nav .line::after */
          width: 50px;
          color: #4a4a4a;
        }
        .circle {
          border-radius: 9999px;
          height: 48px;
          width: 48px;
          border-style: solid;
          border-width: 1px;
          border-color: var(--light-gray-1);
          background-color: white;
          justify-content: center;
          align-items: center;
          display: flex;
          text-align: center;
        }
        :host([color]) .circle {
          border-color: var(--color);
          background-color: var(--color);
          color: #fff;
        }
        :host([color]) .label {
          color: var(--color);
        }
      `,
    ];
  }

  @property({ type: String, reflect: true })
  color?: 'blue' | 'green';

  @property()
  label: string = "";

  render() {
    return html`
      <main>
        <div class="circle">
          <slot></slot>
        </div>
        <p class="label">${this.label}</p>
      </main>
    `;
  }
}
