import React from "react";
import ProductSelect from "./ProductSelect";

/**
 * Represents a Support Form component.
 * Contains product select, subject and message of support ticket.
 *
 * @returns A Support Form component.
 */
export default function SupportForm() {
  const products = [
    { name: "Planner" },
    { name: "Tax" },
    { name: "Law" },
    { name: "3D Models" },
  ];

  return (
    <form className="container form-container">
      <h2>Contact support</h2>
      <p>
        You are signed in as:<br></br>
        <span className="user-email">
          {/* TODO: Add user e-mail */}email@company.com
        </span>
      </p>

      <ProductSelect products={products} />

      <label htmlFor="support-subject">Subject</label>
      <input
        id="support-subject"
        name="support-subject"
        type="text"
        placeholder="Subject"
        required
      ></input>

      <label htmlFor="support-message">Message</label>
      <textarea
        id="support-message"
        name="support-message"
        placeholder="Describe the issue"
        required
      ></textarea>

      <button
        className="default-button submit-button m-t-1"
        type="submit"
        onClick={(event) => validateForm(event)}
      >
        Send
      </button>
      <p className="form-alert"></p>
    </form>
  );
}

/**
 * Confirm that all the form's input is valid.
 *
 * If the user has not selected an option for the product,
 * inform the user that their input is needed.
 *
 * @param event Mouse Event on button
 */
function validateForm(
  event: React.MouseEvent<HTMLButtonElement, MouseEvent>
): void {
  const productSelect: HTMLSelectElement | null =
    document.querySelector("#product-select");
  const formAlert: HTMLElement | null = document.querySelector(".form-alert");

  if (productSelect?.selectedIndex == 0) {
    event.preventDefault();

    if (formAlert != null) {
      formAlert.innerHTML = "Please select a product";
    }
  }
}
