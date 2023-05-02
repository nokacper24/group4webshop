import { MouseEvent as ReactMouseEvent } from "react";
import TermsOfService from "./TermsOfService";

/**
 * Represents the Register User component on the Create Account page.
 * Takes in the user input in a form to create their account.
 *
 * @returns A Register User component.
 */
export default function RegisterUser() {
  return (
    <>
      <p>Fill out all the fields to create your account.</p>

      <form>
        <label htmlFor="create-account_email">E-mail</label>
        <input
          id="create-account_email"
          name="email"
          value="user@company.com" /* TODO: Fill value dynamically */
          required
          disabled
        />

        <label htmlFor="create-account_company-name">Company name</label>
        <input
          id="create-account_company-name"
          name="company-name"
          value="CompanyName" /* TODO: Fill value dynamically */
          type="text"
          required
          disabled
        />

        <label htmlFor="create-account_password">Password</label>
        <input
          id="create-account_password"
          name="password"
          type="password"
          required
        />

        <label htmlFor="create-account_confirm-password">
          Confirm password
        </label>
        <input
          id="create-account_confirm-password"
          name="confirm-password"
          type="password"
          required
        />

        <TermsOfService />

        <p className="form-alert"></p>

        <button
          className="default-button submit-button m-t-1"
          type="submit"
          onClick={(event) => validateForm(event)}
        >
          Register
        </button>
      </form>
    </>
  );
}

/**
 * Confirm that the form has valid input.
 * Check if the password and confirm password fields are identical.
 *
 * @param event Mouse Event on button
 */
function validateForm(
  event: ReactMouseEvent<HTMLButtonElement, MouseEvent>
): void {
  const formAlert: HTMLParagraphElement | null =
    document.querySelector(".form-alert");
  const password: HTMLInputElement | null = document.querySelector(
    "#create-account_password"
  );
  const confirmPassword: HTMLInputElement | null = document.querySelector(
    "#create-account_confirm-password"
  );

  if (password?.value != confirmPassword?.value) {
    event.preventDefault();

    if (formAlert != null) {
      formAlert.innerHTML =
        '"Confirm password" must contain the same value as "Password"';
    }
  }
}
