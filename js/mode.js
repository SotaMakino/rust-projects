const mode = (nums) => {
  const map = new Map();
  let max = { key: 0, value: 0 };

  nums.forEach((n) => {
    if (!map.has(n)) {
      map.set(n, 1);
    } else {
      map.set(n, map.get(n) + 1);
    }
    if (map.get(n) > max.value) {
      max = { key: n, value: map.get(n) };
    }
  });

  return max.key;
};

console.log(mode([1, 5, 3, 3, 5, 7, 10, 3, 3]));
