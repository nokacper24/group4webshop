import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { User } from "../../../Interfaces";
import { fetchUser } from "../../../ApiController";

export default function EditUserAccess() {
  const { userId } = useParams();
  const [user, setUser] = useState<User>();

  useEffect(() => {
    fetchUser(userId!).then((user) => {
      setUser(user);
    });
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>User access</h1>
        <p>
          Enable or disable a user's access to their company's licenses.
          Remember that some licenses might already have max total of users.{" "}
        </p>
        <p>User: {user?.email}</p>
      </section>
    </>
  );
}
