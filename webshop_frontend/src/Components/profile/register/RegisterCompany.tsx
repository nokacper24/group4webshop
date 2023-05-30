import { useEffect, useRef, useState } from "react";
import TermsOfService from "./TermsOfService";
import { getInviteInfo, registerCompany } from "../../../ApiController";

/**
 * Represents the Register Company component on the Create Account page.
 * Takes in the user input in a form to create their account.
 *
 * @returns A Register Company component.
 */
export default function RegisterCompanyAccount() {
  interface InviteInfo {
    email: string;
    companyName: string;
    companyAddress: string;
    role: string;
  }

  const [inviteInfo, setInviteInfo] = useState<InviteInfo | null>(null);
  // get invite info from endpoint
  // if invite info is valid, fill in email field and disable it
  // if invite info is invalid, show error message and redirect to /register/email

  const getInfo = async (id: string) => {
    let result = await getInviteInfo(id);
    if (result.ok) {
      setInviteInfo(await result.json());
    } else {
      console.log("Invalid invite ID");
    }
  };

  const password = useRef<HTMLInputElement>(null);
  const confirmPassword = useRef<HTMLInputElement>(null);
  const [formAlert, setFormAlert] = useState<string>("");

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
    //get the id from the url and call the getInfo function. e.g. /register/company/1234 -> 1234 or /someting/1234 -> 1234
    //get the last part of the url
    const url = window.location.href;
    const id = url.substring(url.lastIndexOf("/") + 1);
    getInfo(id);
  }, []);

  return (
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

        <label htmlFor="create-account_company-address">Company address</label>
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

        <button className="default-button submit-button m-t-1" type="submit">
          Register
        </button>
      </form>
    </>
  );
}
