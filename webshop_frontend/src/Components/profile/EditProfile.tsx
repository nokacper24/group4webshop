import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { MeUser } from "../../Interfaces";
import { patchPartialUser } from "../../ApiController";

interface Props {
  user: MeUser;
}

export default function EditProfile(props: Props) {
  const [user] = useState<MeUser>(props.user);
  const [email, setEmail] = useState<string>(props.user.email);

  const navigate = useNavigate();

  const handleSave = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (email != user?.email) {
      patchPartialUser(user!.user_id.toString(), email).then(
        (response: Response) => {
          if (response.ok) {
            // Refresh
            navigate(0);
          } else {
            alert("Failed to save changes");
          }
        }
      );
    }
  };

  return (
    <>
      <section className="container left-aligned">
        <h1>Edit Profile</h1>
        <form onSubmit={(event) => handleSave(event)}>
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
    </>
  );
}
