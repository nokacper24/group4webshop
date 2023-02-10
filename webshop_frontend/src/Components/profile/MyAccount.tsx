import React from "react";
import LicenseList from "./managing/LicenseList";

/**
 * Represents the My Account page.
 * Contains information about the user acccount and owned licenses.
 *
 * @returns The My Account page component.
 */
export default function MyAccount() {
  return (
    <React.Fragment>
      <section className="container left-aligned">
        <h1>My account</h1>
        <p>
          E-mail: user@company.com
          <button className="default-button small-button">Edit</button>
          <br></br>
          Company: Acme
        </p>
        <p>
          <a href="">Reset password</a>
        </p>
      </section>
      <section className="container left-aligned">
        <h2>Licenses</h2>
        <LicenseList />
      </section>
    </React.Fragment>
  );
}
