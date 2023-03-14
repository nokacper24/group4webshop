import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { SelectTableRowProps } from "../managing/ManageLicenseAccess";
import SelectTable from "./SelectTable";

type UserProps = {
  userId: string;
  email: string;
};

export default function CompanyUsers() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const { companyId } = useParams();

  const [users, setUsers] = useState<SelectTableRowProps[]>([]);
  const [newRemovedUsers] = useState<Set<string>>(new Set());

  const editUser = () => {
    console.log("Editing user...");
  };

  /**
   * Add user as pending to be removed.
   *
   * @param user The user to remove.
   */
  const updateNewRemovedUsers = (user: any) => {
    newRemovedUsers.add(user.id);
  };

  /**
   *  Remove selected users from list of users.
   *
   * @param indices The indices of the users in the list.
   */
  const removeUsers = (indices: number[]) => {
    let sortedIndices = indices.sort((a, b) => a - b);

    for (let i = sortedIndices.length - 1; i >= 0; i--) {
      let index = sortedIndices[i];
      let user = usersList.rows[index];
      usersList.rows = [
        ...usersList.rows.slice(0, index),
        ...usersList.rows.slice(index + 1),
      ];

      updateNewRemovedUsers(user);
    }

    setUsers(usersList.rows);
  };

  const usersList = {
    header: {
      columns: [{ text: "User" }],
    },
    rows: users,
    button: { text: "Edit", action: editUser },
    outsideButtons: [{ text: "Remove selected", action: removeUsers }],
  };

  /**
   * Send a GET request to get the company users.
   *
   * @returns A list of all company users
   */
  const fetchCompanyUsers = async () => {
    const response = await fetch(`${baseUrl}/api/companies/${companyId}/users`);
    const data = await response.json();
    const users: UserProps[] = data.map((user: any) => {
      return {
        userId: user.user_id,
        email: user.email,
      };
    });
    return users;
  };

  const sendDeleteRequest = async () => {
    fetch(`${baseUrl}/api/users`, {
      method: "DELETE",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        users: Array.from(newRemovedUsers, (item) => {
          return {
            user_id: item,
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
      .catch(() => alert("Failed to save users"));
  };

  const handleSave = () => {
    if (newRemovedUsers.size > 0) {
      if (
        confirm(
          "Are you sure you want to remove the users? This action cannot be reversed."
        )
      ) {
        console.log("Sending DELETE request...");
      }
    }
  };

  useEffect(() => {
    fetchCompanyUsers().then((users) => {
      setUsers(
        users.map((user) => {
          return {
            id: user.userId,
            columns: [{ text: user.email }],
          };
        })
      );
    });
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>Manage users</h1>
        <SelectTable
          header={usersList.header}
          rows={usersList.rows}
          button={usersList.button}
          outsideButtons={usersList.outsideButtons}
        />
        <button className="default-button small-button" onClick={handleSave}>
          Save changes
        </button>
      </section>
      <section className="container left-aligned">
        <h1>Add users</h1>
      </section>
    </>
  );
}
