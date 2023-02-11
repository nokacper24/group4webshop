import React, { useRef, useState } from "react";

export type LicenseRowProps = {
  license: {
    name: string;
    activeUsers: number;
    total: number;
    status: string;
    details: string[];
  };
};

/**
 * Represents a Row component in the License List component.
 * Clicking on the expand button will show more information about the license.
 *
 * @param props The license information to be shown in the row.
 * @returns The Row component as a JSX element.
 */
export default function LicenseListRow({ license }: LicenseRowProps) {
  const [collapsed, setCollapsed] = useState(true);
  const toggleVisibility = () => {
    setCollapsed((c) => !c);
  };

  let buttons;
  if (license.status == "Active") {
    buttons = (
      <span className="button-container">
        <button className="default-button small-button">Cancel renewal</button>
        <button className="default-button small-button">Manage access</button>
      </span>
    );
  } else {
    buttons = (
      <span className="button-container">
        <button className="default-button small-button">Manage access</button>
      </span>
    );
  }

  return (
    <React.Fragment>
      <tr className="row-header">
        <td>{license.name}</td>
        <td>{license.activeUsers}</td>
        <td>{license.total}</td>
        <td>{license.status}</td>
        <td>
          <button
            className="icon-button expand-button"
            onClick={toggleVisibility}
          >
            <svg
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
            Active period: {license.details[0]}&ndash;{license.details[1]}
            {buttons}
          </p>
        </td>
      </tr>
    </React.Fragment>
  );
}
