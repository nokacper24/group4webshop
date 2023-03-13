import React, { useState } from "react";
import { Link, useParams } from "react-router-dom";
import { LicenseProps } from "./LicenseList";

type LicenseRowProps = {
  license: LicenseProps;
};

/**
 * Represents a Row component in the License List component.
 * Clicking on the expand button will show more information about the license.
 *
 * @param license The license information to be shown in the row.
 * @returns The Row component as a JSX element.
 */
export default function LicenseListRow({ license }: LicenseRowProps) {
  const { userId } = useParams();
  const [collapsed, setCollapsed] = useState(true);
  const toggleVisibility = () => {
    setCollapsed((c) => !c);
  };

  const cancelButton = (
    <button className="default-button small-button">Cancel renewal</button>
  );
  const manageButton = (
    <Link to={`../manage-license/${license.licenseId}`}>
      <button className="default-button small-button">Manage access</button>
    </Link>
  );

  let buttons;
  if (license.valid == true) {
    buttons = (
      <span className="button-container">
        {cancelButton}
        {manageButton}
      </span>
    );
  } else {
    buttons = <span className="button-container">{manageButton}</span>;
  }

  return (
    <React.Fragment>
      <tr className="row-header">
        <td>{license.productName}</td>
        <td>{0}</td>
        <td>{license.amount}</td>
        <td>{license.valid ? "Valid" : "Invalid"}</td>
        <td>
          <button
            className="icon-button expand-button"
            onClick={toggleVisibility}
          >
            <svg
              transform={`${collapsed ? "" : "scale(1 -1)"}`}
              width="2em"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 150 320 260"
            >
              <title>Expand</title>
              <desc>Expand to show more info</desc>
              {/* Font Awesome Pro 6.3.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. */}
              <path d="M137.4 374.6c12.5 12.5 32.8 12.5 45.3 0l128-128c9.2-9.2 11.9-22.9 6.9-34.9s-16.6-19.8-29.6-19.8L32 192c-12.9 0-24.6 7.8-29.6 19.8s-2.2 25.7 6.9 34.9l128 128z" />
            </svg>
          </button>
        </td>
      </tr>
      <tr className={`row-details ${collapsed ? "collapsed" : ""}`}>
        <td colSpan={5}>
          <p>
            Active period: {license.startDate.toLocaleDateString()} to{" "}
            {license.endDate.toLocaleDateString()}
            {buttons}
          </p>
        </td>
      </tr>
    </React.Fragment>
  );
}
