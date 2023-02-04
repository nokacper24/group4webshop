type ProductSelectProps = {
  products: {
    name: string;
  }[];
};

export default function ProductSelect({ products }: ProductSelectProps) {
  return (
    <select id="product-select" name="products" defaultValue="0">
      <option key="0" value="0" disabled>
        Please choose a product
      </option>

      {products.map((product) => (
        <option key={product.name}>{product.name}</option>
      ))}
    </select>
  );
}
