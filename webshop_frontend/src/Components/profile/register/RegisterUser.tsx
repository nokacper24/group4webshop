import { useEffect, useRef, useState } from "react";
import TermsOfService from "./TermsOfService";
import {
  FetchError,
  getInviteInfo,
  registerCompanyUser,
} from "../../../ApiController";

/**
 * Represents the Register User component on the Create Account page.
 * Takes in the user input in a form to create their account.
 *
 * @returns A Register User component.
 */
export default function RegisterUser() {
  const password = useRef<HTMLInputElement>(null);
  const confirmPassword = useRef<HTMLInputElement>(null);
  const [formAlert, setFormAlert] = useState<string>("");

  const [email, setEmail] = useState<string>("");
  const [companyName, setCompanyName] = useState<string>("companyName");
  const [companyAddress, setCompanyAddress] =
    useState<string>("companyAddress");

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
    let passw0rd = event.currentTarget["password"].value;

    let result = await registerCompanyUser(id, passw0rd);

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
        setCompanyName(invite_info.company_name);
        setCompanyAddress(invite_info.company_address);
        setEmail(invite_info.email);
      })
      .catch((error: FetchError) => {
        console.log(error);
      });
  }, []);

  return (
    <>
      <p>Fill out all the fields to create your account.</p>

      <form onSubmit={(event) => handleSubmit(event)}>
        <label htmlFor="create-account_email">E-mail</label>
        <input
          id="create-account_email"
          name="email"
          value={email}
          required
          disabled
          autoComplete="off"
        />

        <label htmlFor="create-account_company-name">Company name</label>
        <input
          id="create-account_company-name"
          name="company-name"
          value={companyName}
          type="text"
          required
          disabled
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

        <button className="default-button submit-button m-t-1" type="submit">
          Register
        </button>
      </form>
    </>
  );
}
