import React from "react";
import LicenseList from "./managing/LicenseList";

/**
 * Represents the Profile page.
 *
 * Contains information about user account and owned licenses.
 */

/**
 * Represents the Profile page.
 * Contains information about the user acccount and owned licenses.
 *
 * @returns The Profile page component as a JSX element.
 */
export default function Profile() {
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
