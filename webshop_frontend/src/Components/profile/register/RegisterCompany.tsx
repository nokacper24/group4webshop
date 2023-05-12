import { useRef, useState } from "react";
import TermsOfService from "./TermsOfService";

/**
 * Represents the Register Company component on the Create Account page.
 * Takes in the user input in a form to create their account.
 *
 * @returns A Register Company component.
 */
export default function RegisterCompanyAccount() {
  const password = useRef<HTMLInputElement>(null);
  const confirmPassword = useRef<HTMLInputElement>(null);
  const [formAlert, setFormAlert] = useState<string>("");

  /**
   * Handle the submit of the support form. Validates the form data.
   *
   * @param event The form event.
   */
  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (password.current?.value != confirmPassword.current?.value) {
      setFormAlert(
        '"Confirm password" must contain the same value as "Password"'
      );
    } else {
      setFormAlert("");
    }

    // TODO: Send the form info somewhere
  };

  return (
    <>
      <p>Fill out all the fields to create your account.</p>

      <form onSubmit={(event) => handleSubmit(event)}>
        <label htmlFor="create-account_email">E-mail</label>
        <input
          id="create-account_email"
          name="email"
          value="user@company.com" /* TODO: Fill value from URL */
          required
          disabled
        />

        <label htmlFor="create-account_company-name">Company name</label>
        <input
          id="create-account_company-name"
          name="company-name"
          type="text"
          required
        />

        <label htmlFor="create-account_company-address">Company address</label>
        <input
          id="create-account_company-address"
          name="company-address"
          type="text"
          required
        />

        <label htmlFor="create-account_password">Password</label>
        <input
          ref={password}
          id="create-account_password"
          name="password"
          type="password"
          required
        />

        <label htmlFor="create-account_confirm-password">
          Confirm password
        </label>
        <input
          ref={confirmPassword}
          id="create-account_confirm-password"
          name="confirm-password"
          type="password"
          required
        />

        <TermsOfService />

        <p className="form-alert">{formAlert}</p>

        <button className="default-button submit-button m-t-1" type="submit">
          Register
        </button>
      </form>
    </>
  );
}
