{r, _} =
  Enum.map_reduce(0..19, 0, fn y, t ->
    l = Enum.to_list(t..(t + 14))
    {
      if(rem(y, 2) == 1,
       do: l,
       else: Enum.reverse(l)
     ), t + 15}
  end)

IO.inspect(r, charlists: :as_lists)
