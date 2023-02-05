import React from "react";
import { Link } from "react-router-dom";

/**
 * Represents the Register component on the Create Account page.
 *
 * Allows the user to register their e-mail, and receive a
 * verification link to create their account.
 *
 * @returns The Create Account register component as a JSX element.
 */
export function CreateAccountRegister() {
  return (
    <React.Fragment>
      <p>
        Type in your business e-mail and we will send you a registration e-mail
      </p>

      <label htmlFor="register-email">E-mail</label>
      <input
        id="register-email"
        name="email"
        type="text"
        placeholder="user@company.com"
        required
      ></input>

      <Link to="verify">
        <button className="default-button m-t-1" type="submit">
          Send verification
        </button>
      </Link>
    </React.Fragment>
  );
}
