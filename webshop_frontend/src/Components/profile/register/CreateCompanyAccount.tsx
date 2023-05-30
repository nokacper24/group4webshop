import { Routes, Route } from "react-router-dom";
import { RegisterEmail } from "./RegisterEmail";
import { EmailVerify } from "./EmailVerify";
import RegisterCompanyAccount from "./RegisterCompany";
import RegisterUser from "./RegisterUser";
import { checkInvite } from "../../../ApiController";
import { useEffect, useState } from "react";

/**
 * Represents the Create Company Account page.
 *
 * Lets the user register a company account (account that
 * can manage the company), or a normal user account.
 *
 * @returns A Create Company Account page.
 */
export default function CreateCompanyAccount() {
  const [inviteId, setInviteId] = useState<string>("");
  const [inviteType, setInviteType] = useState<string>("");

  const checkInviteType = async (inviteId: string) => {
    if (inviteId === "" || inviteId === undefined || inviteId === null) {
      return;
    }
    const response = await checkInvite(inviteId);
    if (response.ok) {
      const data: any = await response.json();

      setInviteType(data);
      return data;
    } else {
      throw new Error("Could not fetch invite.");
    }
  };

  useEffect(() => {
    const inviteId = window.location.href.split("/").pop();
    setInviteId(inviteId!);
    checkInviteType(inviteId!);
  }, []);

  return (
    <section className="container form-container">
      <h1>Create company account</h1>

      <Routes>
        <Route path="/" element={<RegisterEmail />} />
        <Route path="verify" element={<EmailVerify />} />
        <Route
          path="data/:inviteid"
          element={
            inviteType === "company" ? (
              <RegisterUser />
            ) : (
              <RegisterCompanyAccount />
            )
          }
        ></Route>
      </Routes>
    </section>
  );
}
