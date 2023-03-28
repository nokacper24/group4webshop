export default function EditProfile() {
  const patchUser = async () => {
    console.log("Patching user...");
  };

  const handleSubmit = () => {
    patchUser();
  };

  return (
    <>
      <section className="container left-aligned">
        <h1>Edit Profile</h1>
        <form onSubmit={handleSubmit}>
          <input type="text" placeholder="E-mail"></input>
          <button type="submit">Save</button>
        </form>
      </section>
    </>
  );
}
