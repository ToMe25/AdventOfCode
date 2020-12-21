import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Main {

	public static void main(String[] args) throws IOException {
		File input = new File("input").getAbsoluteFile();
		if (!input.exists()) {
			System.out.println(input.getPath() + " does not exist!");
			System.out.println("trying " + input.getPath() + ".txt instead.");
			input = new File(input.getPath() + ".txt");
			if (!input.exists()) {
				System.err.println("neither " + input.getPath().substring(0, input.getPath().length() - 4) + ",");
				System.err.println("nor " + input.getPath() + " exists!");
				return;
			}
		}

		Map<String, Integer> ingredients = new HashMap<>();
		Map<String, List<String>> allergens = new HashMap<>();
		for (String line : Files.readAllLines(input.toPath())) {
			List<String> ingreds = new ArrayList<>();
			for (String ingredient : line.substring(0, line.indexOf('(') - 1).split(" ")) {
				ingreds.add(ingredient);
				if (ingredients.containsKey(ingredient)) {
					ingredients.put(ingredient, ingredients.get(ingredient) + 1);
				} else {
					ingredients.put(ingredient, 1);
				}
			}

			for (String allergen : line.substring(line.indexOf('(') + 10, line.indexOf(')')).split(", ")) {
				if (!allergens.containsKey(allergen)) {
					allergens.put(allergen, new ArrayList<>(ingreds));
				} else {
					allergens.get(allergen).retainAll(ingreds);
				}
			}
		}

		int allergenFree = 0;
		for (String ingredient : ingredients.keySet()) {
			if (allergens.values().stream().flatMap(list -> list.stream()).noneMatch(ing -> ing.equals(ingredient))) {
				allergenFree += ingredients.get(ingredient);
			}
		}

		System.out.println(
				"All the ingredients that can't be allergens appear a combined total of " + allergenFree + " times.");

		while (allergens.values().stream().anyMatch(ingr -> ingr.size() > 1)) {
			for (List<String> ingreds : allergens.values()) {
				if (ingreds.size() == 1) {
					allergens.values().forEach(ing -> {
						if (ing != ingreds)
							ing.remove(ingreds.get(0));
					});
				}
			}
		}

		final String[] dangerList = new String[] { "" };
		allergens.entrySet().stream().sorted((entry1, entry2) -> entry1.getKey().compareTo(entry2.getKey()))
				.map(entry -> entry.getValue().get(0)).forEach(ingredient -> dangerList[0] += "," + ingredient);
		dangerList[0] = dangerList[0].substring(1);

		System.out.println("The danger list is \"" + dangerList[0] + "\".");
	}
}
