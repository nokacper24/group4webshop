import React, { Component } from "react";
import { Link } from "react-router-dom";

/**
 * Represents the Details component on the Create Account page.
 *
 * Takes in the user input in a form to create their account.
 */
export default class CreateAccountDetails extends Component {
  render() {
    return (
      <React.Fragment>
        <p>Fill out all the fields to create your account.</p>

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

        <div className="checkbox-input">
          <input id="accept-terms" type="checkbox" required />
          <label htmlFor="accept-terms">
            I have read and agree to the <a href="#!">terms of service</a>.
          </label>
        </div>

        <button
          className="default-button submit-button m-t-1"
          type="submit"
          onClick={(event) => this.validateForm(event)}
        >
          Register
        </button>
        <p className="form-alert"></p>
      </React.Fragment>
    );
  }

  componentDidMount(): void {}

  validateForm(event: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
    const formAlert: HTMLParagraphElement | null =
      document.querySelector(".form-alert");
    const password: HTMLInputElement | null = document.querySelector(
      "#create-account_password"
    );
    const confirmPassword: HTMLInputElement | null = document.querySelector(
      "#create-account_confirm-password"
    );

    if (password?.value != confirmPassword?.value && formAlert != null) {
      event.preventDefault();
      formAlert.innerHTML =
        '"Confirm password" must contain the same value as "Password"';
    } else if (password?.value == confirmPassword?.value && formAlert != null) {
      event.preventDefault();
      formAlert.innerHTML = "";
    }
  }
}
