import { useEffect, useRef, useState } from "react";
import TermsOfService from "./TermsOfService";
import { getInviteInfo, registerCompanyUser } from "../../../ApiController";

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
  const [companyName, setCompanyName] = useState<string>("");
  const [companyAddress, setCompanyAddress] = useState<string>("");

  interface InviteInfo {
    email: string;
    companyName: string;
    companyAddress: string;
    role: string;
  }
  const [inviteInfo, setInviteInfo] = useState<InviteInfo | null>(null);

  const getInfo = async (id: string) => {
    let result = await getInviteInfo(id);
    if (result.ok) {
      let resultJson = await result.json();
      setInviteInfo(resultJson);

      setEmail(resultJson.email);
      setCompanyName(resultJson.companyName);
      setCompanyAddress(resultJson.companyAddress);
    } else {
      console.log("Invalid invite ID");
    }
  };

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
      window.location.href = "/login";
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
          value={email}
          required
          disabled
        />

        <label htmlFor="create-account_company-name">Company name</label>
        <input
          id="create-account_company-name"
          name="company-name"
          value={companyName}
          type="text"
          required
          disabled
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
