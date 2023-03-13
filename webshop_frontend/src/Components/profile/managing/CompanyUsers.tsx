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
  const [removedUsers] = useState<Set<string>>(new Set());

  const editUser = () => {
    console.log("Editing user...");
  };

  const usersList = {
    header: {
      columns: [{ text: "User" }],
    },
    rows: users,
    button: { text: "Edit", action: editUser },
    outsideButtons: [{ text: "Remove", action: editUser }],
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
      </section>
    </>
  );
}
