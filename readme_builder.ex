defmodule README.Config do
  def dirs do
    ~w(/algorithms /data_structures /problems)
  end

  def header do
    """
    # Algorithms & Data Structures

    As a shortcut, to avoid browsing all folders, you can simply view any of the table entries and click on the heavy green check mark so view the implementation in a specific language.


    """
  end

  def categories do
    Enum.map(dirs(), &String.replace(&1, "/", ""))
  end

  def langs do
    Enum.sort(~w(c elixir python ruby rust))
  end

  def project_title do
    "# Algorithms & Data Structures"
  end

  def table_vert_sep do
    "|"
  end

  def table_hor_sep do
    "-"
  end

  def done_marker do
    inspect([:heavy_check_mark])
  end
end

defmodule README.Builder do
  import README.Config

  def build_readme() do
    content_for_all_categories =
      for category <- categories(), into: "" do
        build_readme_per_category(category) <> "\n\n"
      end

    header() <> content_for_all_categories
  end

  def build_readme_per_category(category) do
    upper_left_corner_title = String.capitalize(category) <> " / Language"
    lang_col = for lang <- langs(), into: "", do: table_vert_sep() <> String.capitalize(lang)
    title_row = table_vert_sep() <> upper_left_corner_title <> lang_col <> table_vert_sep()
    title_sep = for _n <- 0..length(langs()), into: "", do: table_vert_sep() <> table_hor_sep()
    title_header = title_row <> "\n" <> title_sep <> table_vert_sep()

    items = Enum.sort(File.ls!(category))

    content =
      for item <- items, into: "" do
        implemented_in = File.ls!(category <> "/" <> item)
        implemented_in = Enum.sort(implemented_in)

        row =
          for lang <- langs(), into: "" do
            table_vert_sep() <>
              if lang in implemented_in do
                path_to_implementation(category, lang, item)
              else
                " "
              end
          end

        Macro.camelize(item) <> row <> "\n"
      end

    "# " <> String.capitalize(category) <> "\n" <> title_header <> "\n" <> content
  end

  def path_to_implementation(category, lang, item, marker_func \\ &done_marker/0) do
    "[#{marker_func.()}](#{category}/#{item}/#{lang}/#{item})"
  end
end

readme = README.Builder.build_readme()
File.write!("README.md", readme)
