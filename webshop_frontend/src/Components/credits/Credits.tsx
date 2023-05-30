/**
 * Represents the Credits page.
 *
 * @returns The Credits  component as a JSX element.
 */
export default function Credits() {
  return (
    <>
      <section className="container left-aligned">
        <h2>Credits</h2>
        <p>
          Images used are either generated with{" "}
          <a href="https://openai.com/product/dall-e-2">Dall-E 2</a>,{" "}
          <a href="https://www.midjourney.com/">Midjourney</a> or are public
          domain. Links to public domain images can be found in the list below.
        </p>
        <ul>
          <li>
            Home page hero image: Photo by <a href="https://unsplash.com/photos/bJjsKbToY34">NordWood Themes</a> on{" "}
            <a href="https://unsplash.com/">Unsplash</a>
          </li>
          <li>
            About page hero image: Photo by{" "}
            <a href="https://unsplash.com/photos/Oalh2MojUuk">Jason Goodman</a>{" "}
            on <a href="https://unsplash.com/">Unsplash</a>
          </li>
          <li>
            Testimonial/review author images are taken from{" "}
            <a href="https://randomuser.me/photos" target="_blank">
              Random User Generator
            </a>
          </li>
          <li>
            "Proflex Tax Solutions" main image: Photo by{" "}
            <a href="https://unsplash.com/photos/hpjSkU2UYSU">Carlos Muza</a> on{" "}
            <a href="https://unsplash.com/">Unsplash</a>
          </li>
          <li>
            One of descriptions of "Online Scheduling Software": Photo by{" "}
            <a href="https://unsplash.com/photos/kmikcu4jrsY">Behnam Norouzi</a>{" "}
            on <a href="https://unsplash.com/">Unsplash</a>
          </li>
          <li>
            One of descriptions of "ProFlex Tax Solutions": Photo by{" "}
            <a href="https://unsplash.com/photos/tR0jvlsmCuQ">path digital</a>{" "}
            on <a href="https://unsplash.com/">Unsplash</a>
          </li>
        </ul>
      </section>
    </>
  );
}
