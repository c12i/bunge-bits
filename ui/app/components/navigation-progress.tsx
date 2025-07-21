import { useNavigation } from "@remix-run/react";
import { useEffect, useState } from "react";

export function NavigationProgress() {
  const navigation = useNavigation();
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    if (navigation.state === "loading") {
      setProgress(20);
      const timer = setTimeout(() => setProgress(80), 100);
      return () => clearTimeout(timer);
    } else {
      setProgress(100);
      const timer = setTimeout(() => setProgress(0), 200);
      return () => clearTimeout(timer);
    }
  }, [navigation.state]);

  if (progress === 0) return;

  return (
    <div className="fixed top-0 left-0 right-0 z-50 h-1 bg-gray-200">
      <div
        className="h-full bg-red-800 transition-all duration-300 ease-out"
        style={{ width: `${progress}%` }}
      />
    </div>
  );
}
