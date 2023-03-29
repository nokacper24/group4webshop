import teapot from "../assets/teapot.png";

export default function PageNotFound() {
  return (
    <>
      <section className="container">
        <h1>404 Not Found</h1>
        <p>Sorry, we could not find the page you were looking for!</p>
        <img src={teapot} alt="teapot" width="300" height="170"></img>
      </section>
    </>
  );
}
