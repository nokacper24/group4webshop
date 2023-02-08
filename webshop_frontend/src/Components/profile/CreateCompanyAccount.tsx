import { Component } from "react";
import { Routes, Route } from "react-router-dom";
import { RegisterEmail } from "./RegisterEmail";
import { EmailVerify } from "./EmailVerify";
import RegisterCompany from "./RegisterCompany";
import RegisterUser from "./RegisterUser";

/**
 * Represents the Create Company Account page.
 *
 * Lets the user register their e-mail, tells them to check their
 * e-mail inbox for verification link, and allows them to input
 * company information and user profile password.
 */
export default class CreateCompanyAccount extends Component {
  render() {
    return (
      <div className="center-container">
        <form className="container form-container">
          <h1>Register company account</h1>

          <Routes>
            <Route path="/" element={<RegisterEmail />}></Route>
            <Route path="verify" element={<EmailVerify />}></Route>
            <Route
              path="register-company"
              element={<RegisterCompany />}
            ></Route>
            <Route path="register-user" element={<RegisterUser />} />
          </Routes>
        </form>
      </div>
    );
  }
}
