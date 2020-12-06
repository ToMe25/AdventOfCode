import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.HashSet;
import java.util.Set;

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

		int globalGroupCount = 0;
		int globalCount = 0;
		Set<Byte> groupFoundAny = new HashSet<>();
		Set<Byte> groupFoundEvery = new HashSet<>();
		Set<Byte> found = new HashSet<>();
		boolean newline = false;
		boolean first = true;
		for (byte b : Files.readAllBytes(input.toPath())) {
			if (b == '\n') {
				if (newline) {
					globalCount += groupFoundAny.size();
					globalGroupCount += groupFoundEvery.size();
					groupFoundAny.clear();
					groupFoundEvery.clear();
					first = true;
				} else {
					groupFoundAny.addAll(found);
					if (first) {
						groupFoundEvery.addAll(found);
						first = false;
					} else {
						for (byte a : groupFoundAny) {
							if (!found.contains(a))
								if(groupFoundEvery.contains(a))
									System.out.println((char)a);
								groupFoundEvery.remove(a);
						}
					}
					found.clear();
					newline = true;
				}
			} else if (b == '\r') {
			} else {
				newline = false;
				found.add(b);
			}
		}

		globalCount += groupFoundAny.size();
		globalGroupCount += groupFoundEvery.size();

		System.out.printf("The sum of all group part 1 answers is %d.%n", globalCount);
		System.out.printf("The sum of all group part 2 answers is %d.%n", globalGroupCount);
	}
}
