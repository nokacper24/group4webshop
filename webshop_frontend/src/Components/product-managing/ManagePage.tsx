export default function ManagePage() {
  return (
    <>
      <h2>Manage product</h2>
      <section>
        <form>
          <label htmlFor="product-name">Product name:</label>
          <input type="text" id="product-name" name="product-name" />
          <label htmlFor="product-price">Product price:</label>
          <div>
            <input type="number" id="product-price" name="product-price" />
            <p>kr</p>
          </div>
          <label htmlFor="product-image">Upload header image</label>
          <input
            type="file"
            id="product-image"
            name="product-image"
            accept="image/png, image/jpeg, image/webp"
          />
          <label htmlFor="product-description">Description:</label>
          <textarea
            id="product-description"
            name="product-description"
            rows={10}
            cols={50}
          />
        </form>
      </section>
      <section>

      </section>
      <section>
        <iframe src=""></iframe>
      </section>
      <section>
        <button>Delete product permanently</button>
        <button>Save</button>
      </section>
    </>
  );
}
