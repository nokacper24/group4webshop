import { useEffect, useState } from "react";
import { Product } from "../../Interfaces";
import { ProductCard } from "./ProductCard";
import { fetchProducts } from "../../ApiController";
import Spinner from "../utils/utils";
import { ErrorMessage } from "../ErrorMessage";

export default function Products() {
  const [products, setProducts] = useState<Product[]>([]);

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    fetchProducts()
      .then((products: Product[]) => {
        setProducts(products);
        setLoading(false);
      })
      .catch((error) => {
        setError(error);
        setLoading(false);
      });
  }, []);

  return (
    <>
      <section className="container">
        {loading && <Spinner />}
        {error && <ErrorMessage message={error.message} />}
        {!error && products.length === 0 && !loading && <NoProductsYet />}
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

function NoProductsYet() {
  return (
    <>
      <h1>Oops!</h1>
      <p>
        It seems like we don't have any products yet!
        <br />
        Come backlater...
      </p>
    </>
  );
}