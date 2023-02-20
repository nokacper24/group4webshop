import React, { useState } from "react";
import { useParams } from "react-router-dom";
import SelectTable from "./SelectTable";

/**
 * A Manage License Access page.
 * The license manager can choose which users in their company can
 * access the license.
 *
 * @returns A Manage License Access page component.
 */
export default function ManageLicenseAccess() {
  const { id } = useParams();
  const totalUsers = 10;

  const [usersWithoutAccess, setUsersWithoutAccess] = useState([
    { id: "user 1", columns: [{ text: "user1@companymail.com" }] },
    { id: "user 2", columns: [{ text: "user2@companymail.com" }] },
    { id: "user 3", columns: [{ text: "user3@companymail.com" }] },
    { id: "user 4", columns: [{ text: "user4@companymail.com" }] },
    { id: "user 5", columns: [{ text: "user5@companymail.com" }] },
    { id: "user 6", columns: [{ text: "user6@companymail.com" }] },
  ]);

  const [usersWithAccess, setUsersWithAccess] = useState([
    { id: "user 7", columns: [{ text: "user7@companymail.com" }] },
    { id: "user 8", columns: [{ text: "user8@companymail.com" }] },
    { id: "user 9", columns: [{ text: "user9@companymail.com" }] },
    { id: "user 10", columns: [{ text: "user10@companymail.com" }] },
    { id: "user 11", columns: [{ text: "user11@companymail.com" }] },
    { id: "user 12", columns: [{ text: "user12@companymail.com" }] },
  ]);

  const addUser = (index: number) => {
    /* Get the user to be moved from "without access" to "with access" */
    let user = withoutAccessTable.rows[index];

    /* Remove user from the "without access" list */
    let newWithoutAccessArray = [
      ...withoutAccessTable.rows.slice(0, index),
      ...withoutAccessTable.rows.slice(index + 1),
    ];
    setUsersWithoutAccess(newWithoutAccessArray);

    /* Add user to the "with access" list */
    withAccessTable.rows.push(user);
    setUsersWithAccess(withAccessTable.rows);
  };

  const removeUser = (index: number) => {
    /* Get the user to be moved from "with access" to "without access" */
    let user = withAccessTable.rows[index];

    /* Remove user from the "with access" list */
    let newWithAccessArray = [
      ...withAccessTable.rows.slice(0, index),
      ...withAccessTable.rows.slice(index + 1),
    ];
    setUsersWithAccess(newWithAccessArray);

    /* Add user to the "without access" list */
    withoutAccessTable.rows.push(user);
    setUsersWithoutAccess(withoutAccessTable.rows);
  };

  const addSelectedUsers = (selectedRowsIndices: number[]) => {
    let sortedIndices = selectedRowsIndices.sort();

    for (let i = sortedIndices.length - 1; i >= 0; i--) {
      let index = sortedIndices[i];
      let user = withoutAccessTable.rows[index];
      withoutAccessTable.rows = [
        ...withoutAccessTable.rows.slice(0, index),
        ...withoutAccessTable.rows.slice(index + 1),
      ];

      withAccessTable.rows.push(user);
    }
    setUsersWithoutAccess(withoutAccessTable.rows);
    setUsersWithAccess(withAccessTable.rows);
  };

  const removeSelectedUsers = (selectedRowsIndices: number[]) => {
    let sortedIndices = selectedRowsIndices.sort();

    for (let i = sortedIndices.length - 1; i >= 0; i--) {
      let index = sortedIndices[i];
      let user = withAccessTable.rows[index];
      withAccessTable.rows = [
        ...withAccessTable.rows.slice(0, index),
        ...withAccessTable.rows.slice(index + 1),
      ];

      withoutAccessTable.rows.push(user);
    }

    setUsersWithAccess(withAccessTable.rows);
    setUsersWithoutAccess(withoutAccessTable.rows);
  };

  const withoutAccessTable = {
    header: {
      columns: [{ text: "Users" }, { text: "Access" }],
    },
    rows: usersWithoutAccess,
    button: { text: "Add", action: addUser },
    outsideButtons: [{ text: "Add all selected", action: addSelectedUsers }],
  };

  const withAccessTable = {
    header: {
      columns: [{ text: "Users" }, { text: "Access" }],
    },
    rows: usersWithAccess,
    button: { text: "Remove", action: removeUser },
    outsideButtons: [
      { text: "Remove all selected", action: removeSelectedUsers },
    ],
  };

  return (
    <React.Fragment>
      <section className="container left-aligned">
        <h1>Manage license access</h1>
        <p>
          {/* TODO: Fetch product name of license */}
          License {id}
        </p>
      </section>

      <section className="container left-aligned">
        <h2>Users without access</h2>

        <SelectTable
          header={withoutAccessTable.header}
          rows={withoutAccessTable.rows}
          button={withoutAccessTable.button}
          outsideButtons={withoutAccessTable.outsideButtons}
        />
      </section>

      <section className="container left-aligned">
        <h2>Users with access</h2>
        <SelectTable
          header={withAccessTable.header}
          rows={withAccessTable.rows}
          button={withAccessTable.button}
          outsideButtons={withAccessTable.outsideButtons}
        />
      </section>
      <section className="container left-aligned button-container">
        <button className="default-button small-button">Save changes</button>
        <p className="table-text">
          New active users:{" "}
          <span
            className={`active-users ${
              totalUsers < usersWithAccess.length ? "text-danger" : ""
            }`}
          >
            {usersWithAccess.length}
          </span>
        </p>
      </section>
    </React.Fragment>
  );
}
