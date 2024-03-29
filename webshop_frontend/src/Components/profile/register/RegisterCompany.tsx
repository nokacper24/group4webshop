import { useEffect, useRef, useState } from "react";
import TermsOfService from "./TermsOfService";
import {
  FetchError,
  getInviteInfo,
  registerCompany,
} from "../../../ApiController";
import { InviteInfo } from "../../../Interfaces";
import { Link } from "react-router-dom";

/**
 * Represents the Register Company component on the Create Account page.
 * Takes in the user input in a form to create their account.
 *
 * @returns A Register Company component.
 */
export default function RegisterCompanyAccount() {
  const [inviteInfo, setInviteInfo] = useState<InviteInfo | null>(null);
  // get invite info from endpoint
  // if invite info is valid, fill in email field and disable it
  // if invite info is invalid, show error message and redirect to /register/email

  const password = useRef<HTMLInputElement>(null);
  const confirmPassword = useRef<HTMLInputElement>(null);
  const [formAlert, setFormAlert] = useState<string>("");
  const [error, setError] = useState<Error | null>(null);

  /**
   * Handle the submit of the support form. Validates the form data.
   *
   * @param event The form event.
   */
  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (password.current?.value != confirmPassword.current?.value) {
      setFormAlert(
        '"Confirm password" must contain the same value as "Password"'
      );
    } else {
      setFormAlert("");
    }

    // send data to endpoint
    let url = window.location.href;
    let id = url.substring(url.lastIndexOf("/") + 1);
    let companyName = event.currentTarget["company-name"].value;
    let companyAddress = event.currentTarget["company-address"].value;
    let passw0rd = event.currentTarget["password"].value;

    let result = await registerCompany(
      id,
      passw0rd,
      companyName,
      companyAddress
    );

    if (result.ok) {
      window.location.href = "/profile";
    } else {
      setFormAlert("Something went wrong. Please try again.");
    }
  };

  useEffect(() => {
    const url = window.location.href;
    const id = url.substring(url.lastIndexOf("/") + 1);
    getInviteInfo(id)
      .then((invite_info) => {
        setInviteInfo(invite_info);
      })
      .catch((error: FetchError) => {
        console.log(error);
        setError(error);
      });
  }, []);

  return (
    <>
      {!error && (
        <>
          <p>Fill out all the fields to create your account.</p>

          <form onSubmit={(event) => handleSubmit(event)}>
            <label htmlFor="create-account_email">E-mail</label>
            <input
              id="create-account_email"
              name="email"
              value={inviteInfo?.email}
              required
              disabled
              autoComplete="off"
            />

            <label htmlFor="create-account_company-name">Company name</label>
            <input
              id="create-account_company-name"
              name="company-name"
              type="text"
              required
              autoComplete="off"
            />

            <label htmlFor="create-account_company-address">
              Company address
            </label>
            <input
              id="create-account_company-address"
              name="company-address"
              type="text"
              required
              autoComplete="off"
            />

            <label htmlFor="create-account_password">Password</label>
            <input
              ref={password}
              id="create-account_password"
              name="password"
              type="password"
              required
              autoComplete="off"
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
              autoComplete="off"
            />

            <TermsOfService />

            <p className="form-alert">{formAlert}</p>

            <button
              className="default-button submit-button m-t-1"
              type="submit"
            >
              Register
            </button>
          </form>
        </>
      )}
      {error && (
        <>
          <p>Something went wrong.</p>
          <Link className="hero-button" to="/home">
            Back to home
          </Link>
        </>
      )}
    </>
  );
}
