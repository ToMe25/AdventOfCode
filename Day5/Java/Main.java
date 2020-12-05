import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

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

		List<Integer> found = new ArrayList<>();
		for (String line : Files.readAllLines(input.toPath())) {
			line = line.replaceAll("[FL]", "0").replaceAll("[RB]", "1");

			found.add(Integer.parseInt(line, 2));
		}

		Collections.sort(found);
		System.out.println("Highest found seat id is " + found.get(found.size() - 1) + ".");

		int last = 0;
		for (int seat : found) {
			if (seat == last + 2) {
				last = seat - 1;
				break;
			}
			last = seat;
		}

		System.out.println("Your seat has id " + last + ".");
	}
}
