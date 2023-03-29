import { useEffect, useState } from "react";
import { Product } from "../../Interfaces";
import { ProductCard, ProductCardProps } from "./ProductCard";

let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
// check if we are in production mode
if (import.meta.env.PROD) {
  baseUrl = "";
}

export default function Products() {
  const [products, setProducts] = useState<Product[]>([]);

  const fetchProducts = async () => {
    const response = await fetch(`${baseUrl}/api/products`);
    const data = await response.json();
    const products = data.map((product: Product) => product);

    setProducts(products);
  };

  useEffect(() => {
    fetchProducts();
  }, []);

  return (
    <>
      <section className="container">
        <h1>Our solutions</h1>
        <ul className="product-list grid-container">
          {products.map((product) => (
            <ProductCard key={product.display_name} product={product} />
          ))}
        </ul>
      </section>
    </>
  );
}
