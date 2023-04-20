import { useEffect, useState } from "react";
import { Product } from "../../Interfaces";
import { ProductCard, ProductCardProps } from "./ProductCard";
import { fetchProducts } from "../../ApiController";
import Spinner from "../utils/utils";

export default function Products() {
  const [products, setProducts] = useState<Product[]>([]);

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    let response = fetchProducts();
    response
      .then((data) => {
        setProducts(data);
      })
      .catch((error) => {
        setError(error);
      })
      .finally(() => {
        setLoading(false);
      });
  }, []);

  return (
    <>
      <section className="container">
        {loading && <Spinner />}
        {error && <p>{error.message}</p>}
        {products.length === 0 && !loading && <p>No products found.</p>}
        {products.length > 0 && !loading && (
          <>
            <h1>Our solutions</h1>
            <ul className="product-list grid-container">
              {products.map((product) => (
                <ProductCard key={product.display_name} product={product} />
              ))}
            </ul>
          </>
        )}
      </section>
    </>
  );
}
