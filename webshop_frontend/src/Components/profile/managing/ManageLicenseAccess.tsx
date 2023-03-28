import { useState, useEffect, useRef } from "react";
import { useParams } from "react-router-dom";
import { License, Product, User } from "../../../Interfaces";
import SelectTable, {
  SelectTableProps,
  SelectTableRowProps,
} from "./SelectTable";
import { createSelectTableProps, createRowProps } from "./SelectTableFunctions";

/**
 * A Manage License Access page.
 * The license manager can choose which users in their company can
 * access the license.
 *
 * @returns A Manage License Access page component.
 */
export default function ManageLicenseAccess() {
  const { licenseId } = useParams();
  const [license, setLicense] = useState<License>({
    license_id: 0,
    valid: false,
    start_date: new Date("1970-01-01"),
    end_date: new Date("1970-01-01"),
    amount: 0,
    company_id: 0,
    product_id: "0",
    product_name: "",
  });
  const [users, setUsers] = useState<User[]>([]);

  const [newUsersWithoutAccess] = useState<Map<string, string>>(new Map());
  const [newUsersWithAccess] = useState<Map<string, string>>(new Map());

  const [usersWithoutAccess, setUsersWithoutAccess] = useState<
    SelectTableRowProps[]
  >([]);
  const [usersWithAccess, setUsersWithAccess] = useState<SelectTableRowProps[]>(
    []
  );

  /**
   * Update if the user access has been changed (from original) when adding user.
   *
   * @param user The user to check if their access has changed.
   */
  const updateChangedOnAdd = (user: SelectTableRowProps) => {
    if (newUsersWithoutAccess.has(user.id)) {
      newUsersWithoutAccess.delete(user.id);
    } else if (!newUsersWithAccess.has(user.id)) {
      newUsersWithAccess.set(user.id, user.columns[0].text);
    }
  };

  /**
   * Update if the user access has been changed (from original) when removing user.
   *
   * @param user The user to check if their access has changed.
   */
  const updateChangedOnRemove = (user: SelectTableRowProps) => {
    if (newUsersWithAccess.has(user.id)) {
      newUsersWithAccess.delete(user.id);
    } else if (!newUsersWithoutAccess.has(user.id)) {
      newUsersWithoutAccess.set(user.id, user.columns[0].text);
    }
  };

  /**
   * Add a user to the list of users with license access.
   *
   * @param index The index of the user in the list of users without
   *        access to be added to the list of users with access.
   */
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

    updateChangedOnAdd(user);
  };

  /**
   * Remove a user from the list of users with license access.
   *
   * @param index The index of the user in the list of users with
   *        access to be added to the list of users without access.
   */
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

    updateChangedOnRemove(user);
  };

  /**
   * Add all selected users to the list of users with license access.
   *
   * @param selectedRowsIndices The indices of the users in the list of users without
   *        access to be added to the list of users with access.
   */
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

      updateChangedOnAdd(user);
    }
    setUsersWithoutAccess(withoutAccessTable.rows);
    setUsersWithAccess(withAccessTable.rows);
  };

  /**
   * Remove all selected users to the list of users with license access.
   *
   * @param selectedRowsIndices The indices of the users in the list of users with
   *        access to be added to the list of users without access.
   */
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

      updateChangedOnRemove(user);
    }

    setUsersWithAccess(withAccessTable.rows);
    setUsersWithoutAccess(withoutAccessTable.rows);
  };

  const withoutAccessTable: SelectTableProps = createSelectTableProps(
    ["Users"],
    usersWithoutAccess,
    "Add",
    addUserAccess,
    new Map([["Add selected", addSelectedUsersAccess]])
  );

  const withAccessTable: SelectTableProps = createSelectTableProps(
    ["Users"],
    usersWithAccess,
    "Remove",
    removeUserAccess,
    new Map([["Remove selected", removeSelectedUsersAccess]])
  );

  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const isInitialMount = useRef(true);

  /**
   * Send a GET request to get a product.
   *
   * @param productId The ID of the product.
   * @returns The product object.
   */
  const fetchProduct = async (productId: string) => {
    const response = await fetch(`${baseUrl}/api/products/${productId}`);
    const data = await response.json();
    const product: Product = data;
    return product;
  };

  /**
   * Send a GET request to get the license information.
   *
   * @returns The license object
   */
  const fetchLicense = async () => {
    const response = await fetch(`${baseUrl}/api/licenses/${licenseId}`);
    const data = await response.json();
    const license: License = data;
    return license;
  };

  /**
   * Send a GET request to get the company users.
   *
   * @param companyId The ID of the company
   * @returns A list of all company users
   */
  const fetchCompanyUsers = async (companyId: number) => {
    const response = await fetch(`${baseUrl}/api/companies/${companyId}/users`);
    const data = await response.json();
    const users: User[] = data.map((user: User) => user);
    return users;
  };

  /**
   * Send a GET request to get the users with access to the license.
   *
   * @returns A list of users with license access.
   */
  const fetchUsersWithAccess = async () => {
    const response = await fetch(`${baseUrl}/api/licenses/${licenseId}/users`);
    const data = await response.json();
    const users: User[] = data.map((user: User) => user);
    return users;
  };

  /**
   * Send a POST request to add users' access to the license.
   */
  const sendAddUsersRequest = () => {
    if (newUsersWithAccess.size > 0) {
      fetch(`${baseUrl}/api/license_users`, {
        method: "POST",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          users: Array.from(newUsersWithAccess, (item) => {
            return {
              user_id: item[0],
              license_id: licenseId ? parseInt(licenseId) : NaN,
            };
          }),
        }),
      })
        .then((response) => {
          const status = response.status;
          if (status == 201) {
            location.reload();
          } else if (status == 400) {
            alert("Failed to save changes, because users already have access");
          } else {
            alert("Something went wrong when saving users");
          }
        })
        .catch(() => alert("Failed to save license access for users"));
    }
  };

  /**
   * Send a DELETE request to remove users' access to the license.
   */
  const sendRemoveUsersRequest = () => {
    if (newUsersWithoutAccess.size > 0) {
      fetch(`${baseUrl}/api/license_users`, {
        method: "DELETE",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          users: Array.from(newUsersWithoutAccess, (item) => {
            return {
              user_id: item[0],
              license_id: licenseId ? parseInt(licenseId) : NaN,
            };
          }),
        }),
      })
        .then((response) => {
          const status = response.status;
          if (status == 200) {
            location.reload();
          } else {
            alert("Something went wrong when saving users");
          }
        })
        .catch(() => alert("Failed to save license access for users"));
    }
  };

  const handleSave = () => {
    if (usersWithAccess.length <= license.amount) {
      sendAddUsersRequest();
      sendRemoveUsersRequest();
    } else {
      alert(
        "You cannot exceed the license user amount. Please select fewer users to give access to."
      );
    }
  };

  useEffect(() => {
    // Get license and users
    fetchLicense()
      .then((license) => {
        setLicense(license);
        fetchProduct(license.product_id).then((product) => {
          license.product_name = product.display_name;
        });
        fetchCompanyUsers(license.company_id).then((users) => setUsers(users));
      })
      .catch((e) => alert("Failed to get license or users"));
  }, []);

  useEffect(() => {
    if (isInitialMount.current) {
      isInitialMount.current = false;
    } else {
      // Sort users between those with and without license access
      fetchUsersWithAccess()
        .then((x) => {
          let withoutAccess: User[] = [];
          let withAccess: User[] = [];

          withoutAccess = users.filter(
            (arr1) => !x.find((arr2) => arr2.user_id === arr1.user_id)
          );
          withAccess = x;

          setUsersWithoutAccess(
            withoutAccess.map((user) => {
              return createRowProps(user.user_id, [user.email]);
            })
          );
          setUsersWithAccess(
            withAccess.map((user) => {
              return createRowProps(user.user_id, [user.email]);
            })
          );
        })
        .catch(() => alert("Failed to fetch users with license access"));
    }
  }, [users]);

  return (
    <>
      <section className="container left-aligned">
        <h1>Manage license access</h1>
        <p>
          Product: {license.product_name}
          <br></br>
          Active users: {usersWithAccess.length - newUsersWithAccess.size}
          <br></br>
          Total allowed: {license.amount}
          <br></br>
          Start date: {new Date(license.start_date).toDateString()}
          <br></br>
          End date: {new Date(license.end_date).toDateString()}
          <br></br>
          Status: {license.valid ? "Valid" : "Invalid"}
          <br></br>
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
        <button className="default-button small-button" onClick={handleSave}>
          Save changes
        </button>
        <p className="table-text">
          New active users:{" "}
          <span
            className={`active-users ${
              license.amount < usersWithAccess.length ? "text-danger" : ""
            }`}
          >
            {usersWithAccess.length}
          </span>
        </p>
      </section>
    </>
  );
}
