import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { MeUser, User } from "../../Interfaces";
import {
  fetchUser,
  resetPassword,
  patchPartialUser,
} from "../../ApiController";

interface Props {
  user: MeUser;
}

export default function EditProfile(props: Props) {
  const [user, setUser] = useState<MeUser>();
  const [email, setEmail] = useState<string>("");

  const navigate = useNavigate();

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (email != user?.email) {
      patchPartialUser(user!.user_id.toString(), email).then(
        (response: Response) => {
          if (response.status == 200) {
            // Refresh
            navigate(0);
          } else {
            alert("Failed to save changes");
          }
        }
      );
    }
  };

  const handlePasswordReset = () => {
    resetPassword(user!.email).then((response) => {
      if (response.status === 200) {
        alert(
          "We have sent you an e-mail with a link to reset password. It may take a few minutes. Check the spam folder if you do not see it."
        );
      } else {
        alert("Failed to reset password.");
      }
    });
  };

  useEffect(() => {
    setUser(props.user);
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>Edit Profile</h1>
        <form onSubmit={(event) => handleSubmit(event)}>
          {/* Prevent implicit submission of the form */}
          <button
            type="submit"
            disabled
            style={{ display: "none" }}
            aria-hidden="true"
          ></button>

          <label>
            E-mail
            <input
              type="text"
              style={{ width: "min(30em, 70vw)" }}
              placeholder="E-mail"
              defaultValue={user?.email}
              onChange={(event) => setEmail(event.target.value)}
            ></input>
          </label>

          <div className="button-container">
            <button type="submit" className="default-button small-button">
              Save
            </button>
          </div>
        </form>
      </section>
      <section className="container left-aligned">
        <h1>Change password</h1>
        <button
          className="default-button small-button"
          onClick={() => handlePasswordReset}
        >
          Reset password
        </button>
      </section>
    </>
  );
}
