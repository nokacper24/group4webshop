import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import SelectTable from "./SelectTable";
import { UserProps, LicenseProps } from "../MyAccount";

/**
 * A Manage License Access page.
 * The license manager can choose which users in their company can
 * access the license.
 *
 * @returns A Manage License Access page component.
 */
export default function ManageLicenseAccess() {
  const { licenseId } = useParams();
  const [license, setLicense] = useState<LicenseProps>();
  const [users, setUsers] = useState<UserProps>();
  const totalUsers = 4;

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

  const addUserAccess = (index: number) => {
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

  const removeUserAccess = (index: number) => {
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

  const addSelectedUsersAccess = (selectedRowsIndices: number[]) => {
    let sortedIndices = selectedRowsIndices.sort((a, b) => a - b);

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

  const removeSelectedUsersAccess = (selectedRowsIndices: number[]) => {
    let sortedIndices = selectedRowsIndices.sort((a, b) => a - b);

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
    button: { text: "Add", action: addUserAccess },
    outsideButtons: [
      { text: "Add all selected", action: addSelectedUsersAccess },
    ],
  };

  const withAccessTable = {
    header: {
      columns: [{ text: "Users" }, { text: "Access" }],
    },
    rows: usersWithAccess,
    button: { text: "Remove", action: removeUserAccess },
    outsideButtons: [
      { text: "Remove all selected", action: removeSelectedUsersAccess },
    ],
  };

  const baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;

  const fetchLicense = async () => {
    const response = await fetch(`${baseUrl}/api/licenses/${licenseId}`);
    const data = await response.json();
    const license = {
      licenseId: data.license_id,
      valid: data.valid,
      startDate: data.start_date,
      endDate: data.end_date,
      amount: data.amount,
      companyId: data.company_id,
      productId: data.product_id,
    };
    setLicense(license);
    return license;
  };

  const fetchCompanyUsers = async (companyId: string) => {
    const response = await fetch(`${baseUrl}/api/company/${companyId}/users`);
    const data = await response.json();
    const users = data.map((user: any) => {
      return {
        userId: user.user_id,
        email: user.email,
      };
    });
    setUsers(users);
  };

  useEffect(() => {
    fetchLicense()
      .then((license) => fetchCompanyUsers(license.companyId))
      .catch((e) =>
        console.log("An error occurred while trying to get the license.", e)
      );
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>Manage license access</h1>
        <p>
          {/* TODO: Fetch product name of license */}
          {license?.productId}
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
    </>
  );
}
