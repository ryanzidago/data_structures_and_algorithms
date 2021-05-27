defmodule StableMarriageTest do
  use ExUnit.Case

  test "match/2" do
    men = %{
      "a" => ~w(o m n l p),
      "b" => ~w(p n m l o),
      "c" => ~w(m p l o n),
      "d" => ~w(p m o n l),
      "e" => ~w(o l m n p)
    }

    women = %{
      "l" => ~w(d b e c a),
      "m" => ~w(b a d c e),
      "n" => ~w(a c e d b),
      "o" => ~w(d a c b e),
      "p" => ~w(b e a c d)
    }

    matches = StableMarriage.match(men, women)

    assert [
             {"a", "o"},
             {"b", "p"},
             {"c", "n"},
             {"d", "m"},
             {"e", "l"}
           ] = matches

    men = %{
      "abe" => ~w(abi eve cath ivy jan dee fay bea hope gay),
      "bob" => ~w(cath hope abi dee eve fay bea jan ivy gay),
      "col" => ~w(hope eve abi dee bea fay ivy gay cath jan),
      "dan" => ~w(ivy fay dee gay hope eve jan bea cath abi),
      "ed" => ~w(jan dee bea cath fay eve abi ivy hope gay),
      "fred" => ~w(bea abi dee gay eve ivy cath jan hope fay),
      "gav" => ~w(gay eve ivy bea cath abi dee hope jan fay),
      "hal" => ~w(abi eve hope fay ivy cath jan bea gay dee),
      "ian" => ~w(hope cath dee gay bea abi fay ivy jan eve),
      "jon" => ~w(abi fay jan gay eve bea dee cath ivy hope)
    }

    women = %{
      "abi" => ~w(bob fred jon gav ian abe dan ed col hal),
      "bea" => ~w(bob abe col fred gav dan ian ed jon hal),
      "cath" => ~w(fred bob ed gav hal col ian abe dan jon),
      "dee" => ~w(fred jon col abe ian hal gav dan bob ed),
      "eve" => ~w(jon hal fred dan abe gav col ed ian bob),
      "fay" => ~w(bob abe ed ian jon dan fred gav col hal),
      "gay" => ~w(jon gav hal fred bob abe col ed dan ian),
      "hope" => ~w(gav jon bob abe ian dan hal ed col fred),
      "ivy" => ~w(ian col hal gav fred bob abe ed jon dan),
      "jan" => ~w(ed hal gav abe bob jon col ian fred dan)
    }

    matches = StableMarriage.match(men, women)

    assert length(Map.keys(men)) == length(matches)

    assert {"jon", "abi"} in matches
    assert {"fred", "bea"} in matches
    assert {"bob", "cath"} in matches
    assert {"col", "dee"} in matches
    assert {"hal", "eve"} in matches
    assert {"dan", "fay"} in matches
    assert {"gav", "gay"} in matches
    assert {"ian", "hope"} in matches
    assert {"abe", "ivy"} in matches
    assert {"ed", "jan"} in matches
  end
end
