import { Routes, Route } from "react-router-dom";
import { RegisterEmail } from "./RegisterEmail";
import { EmailVerify } from "./EmailVerify";
import RegisterCompanyAccount from "./RegisterCompany";
import RegisterUser from "./RegisterUser";

/**
 * Represents the Create Company Account page.
 *
 * Lets the user register a company account (account that
 * can manage the company), or a normal user account.
 *
 * @returns A Create Company Account page.
 */
export default function CreateCompanyAccount() {
  return (
    <section className="container form-container">
      <h1>Create company account</h1>

      <Routes>
        <Route path="/" element={<RegisterEmail />} />
        <Route path="verify" element={<EmailVerify />} />
        <Route path="register-company" element={<RegisterCompanyAccount />} />
        <Route path="register-user" element={<RegisterUser />} />
      </Routes>
    </section>
  );
}
