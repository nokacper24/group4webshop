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
  const [usersWithoutAccess, setUsersWithoutAccess] = useState([
    { id: "user 1", columns: [{ text: "email 1" }] },
    { id: "user 2", columns: [{ text: "email 2" }] },
    { id: "user 3", columns: [{ text: "email 3" }] },
    { id: "user 4", columns: [{ text: "email 4" }] },
  ]);

  const [usersWithAccess, setUsersWithAccess] = useState([
    { id: "user 5", columns: [{ text: "email 5" }] },
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
    console.log("Removing user... " + index);
  };

  const withoutAccessTable = {
    header: {
      columns: [{ text: "Users" }, { text: "Access" }],
    },
    rows: usersWithoutAccess,
    button: { text: "Add", action: addUser },
    outsideButtons: [{ text: "Add all selected" }],
  };

  const withAccessTable = {
    header: {
      columns: [{ text: "Users" }, { text: "Access" }],
    },
    rows: usersWithAccess,
    button: { text: "Remove", action: removeUser },
    outsideButtons: [{ text: "Remove all selected" }],
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
    </React.Fragment>
  );
}
