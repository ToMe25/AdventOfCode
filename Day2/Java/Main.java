import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

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

		int correctPwds1 = 0;
		int correctPwds2 = 0;
		for (String line : Files.readAllLines(input.toPath())) {
			String[] inputs = line.split(" ");
			String[] range = inputs[0].split("-");
			int min = Integer.parseInt(range[0]);
			int max = Integer.parseInt(range[1]);
			char requested = inputs[1].charAt(0);
			int occurrences = (int) inputs[2].chars().filter(c -> c == requested).count();
			if (occurrences >= min && occurrences <= max) {
				correctPwds1++;
			}

			if ((inputs[2].charAt(min - 1) == requested) != (inputs[2].charAt(max - 1) == requested)) {
				correctPwds2++;
			}
		}

		System.out.println("Found " + correctPwds1 + " correct step 1 passwords.");
		System.out.println("Found " + correctPwds2 + " correct step 2 passwords.");
	}

}
